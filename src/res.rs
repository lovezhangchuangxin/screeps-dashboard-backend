use crate::{
    constants::{
        B_BLUE_RES, B_GREEN_RES, B_GREY_RES, B_PINK_RES, B_WHITE_RES, B_YELLOW_RES, BARS_RES,
        BASE_RES, C_BLUE_RES, C_GREEN_RES, C_GREY_RES, C_PINK_RES, C_YELLOW_RES, POWER_RES,
        res_color_map,
    },
    utils::{draw_res, draw_res_text, merge_res, parse_color},
};
use chrono::prelude::*;
use plotters::prelude::*;
use screeps_rust_api::{RoomObject, ScreepsApi, ScreepsError, ScreepsResult};
use std::collections::HashMap;

/// 查询玩家指定shard具有的资源
/// 参数：
/// - username: 玩家名称
/// - target_shard: 目标 shard，传 `all` 表示所有 shard
pub async fn query_res(
    api: &ScreepsApi,
    username: &str,
    target_shard: &str,
) -> ScreepsResult<HashMap<String, HashMap<String, i32>>> {
    let mut result = HashMap::new();

    // 先根据玩家信息查玩家的 id
    let user_info = api.get_user_info_by_name(username).await?;
    if user_info.base_data.ok.unwrap() != 1 {
        return Err(ScreepsError::Api("玩家不存在".to_string()));
    }

    let user_id = user_info.user.unwrap()._id;
    // 再根据玩家 id 查玩家所有房间
    let user_rooms = api.get_user_rooms(&user_id).await?;
    if user_rooms.base_data.ok.unwrap() != 1 {
        return Err(ScreepsError::Api("玩家没有房间".to_string()));
    }

    // 收集所有需要查询的房间和 shard 信息
    let mut room_shard_pairs = Vec::new();
    for (shard, rooms) in user_rooms.shards.unwrap().iter() {
        if target_shard != "all" && shard != target_shard {
            continue;
        }
        for room in rooms {
            room_shard_pairs.push((room.clone(), shard.clone()));
        }
    }

    // 创建所有 future
    let futures: Vec<_> = room_shard_pairs
        .iter()
        .map(|(room, shard)| api.get_room_objects(room, shard))
        .collect();

    // 执行所有请求
    let responses = futures::future::join_all(futures).await;
    // 处理响应
    for (response, (room, shard)) in responses.into_iter().zip(room_shard_pairs.iter()) {
        match response {
            Ok(room_objects) => {
                if room_objects.base_data.ok.unwrap() != 1 {
                    eprintln!(
                        "Failed to fetch objects for room {} in shard {}, reason: {}",
                        room,
                        shard,
                        room_objects.base_data.error.unwrap()
                    );
                    continue;
                }
                let shard_res_map = result.entry(shard.clone()).or_insert_with(HashMap::new);
                for room_object in room_objects.objects.unwrap() {
                    match room_object {
                        RoomObject::Storage(storage) => {
                            for (resource_type, amount) in storage.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        RoomObject::Terminal(terminal) => {
                            for (resource_type, amount) in terminal.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        RoomObject::Factory(link) => {
                            for (resource_type, amount) in link.store.iter() {
                                let amount = amount.unwrap_or(0);
                                shard_res_map
                                    .entry(resource_type.to_string())
                                    .and_modify(|a| *a += amount)
                                    .or_insert(amount);
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Failed to fetch objects for room {} in shard {}: {}",
                    room, shard, e
                );
                return Err(e);
            }
        }
    }

    Ok(result)
}

/// 绘制资源数据为图片
pub async fn draw_res_image(
    api: &ScreepsApi,
    username: &str,
    target_shard: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let res = query_res(api, username, target_shard).await?;
    let res = merge_res(&res);
    let image_path = format!("data/{}_{}.png", username, target_shard);
    let gap = 100;
    let res_color_map = res_color_map();
    let root = BitMapBackend::new(&image_path, (9 * gap + 30, 540)).into_drawing_area();
    root.fill(&parse_color("#2b2b2b").unwrap())?;
    draw_res_text(&root, "baseRes", 10, 15, "#ffffff");
    BASE_RES.iter().enumerate().for_each(|(i, &name)| {
        draw_res(
            &root,
            &res_color_map,
            name,
            res.get(name).unwrap_or(&0),
            30 + gap * (i as u32),
            30,
        );
    });

    draw_res_text(&root, "barsRes", 10, 65, "#ffffff");
    BARS_RES.iter().enumerate().for_each(|(i, &name)| {
        draw_res(
            &root,
            &res_color_map,
            name,
            res.get(name).unwrap_or(&0),
            30 + gap * (i as u32),
            80,
        );
    });

    draw_res_text(&root, "powerRes", 10, 115, "#ffffff");
    POWER_RES.iter().enumerate().for_each(|(i, &name)| {
        draw_res(
            &root,
            &res_color_map,
            name,
            res.get(name).unwrap_or(&0),
            30 + gap * (i as u32),
            130,
        );
    });

    draw_res_text(&root, "goods", 10, 165, "#ffffff");
    let goods: Vec<Box<[&str]>> = vec![
        Box::new(C_GREY_RES),
        Box::new(C_BLUE_RES),
        Box::new(C_YELLOW_RES),
        Box::new(C_PINK_RES),
        Box::new(C_GREEN_RES),
    ];
    for (y, goods) in goods.iter().enumerate() {
        goods.iter().enumerate().for_each(|(i, &name)| {
            draw_res(
                &root,
                &res_color_map,
                name,
                res.get(name).unwrap_or(&0),
                30 + gap * (i as u32),
                180 + (y as u32) * 30,
            );
        });
    }

    draw_res_text(&root, "labRes", 10, 335, "#ffffff");
    let goods: Vec<Box<[&str]>> = vec![
        Box::new(B_GREY_RES),
        Box::new(B_BLUE_RES),
        Box::new(B_YELLOW_RES),
        Box::new(B_PINK_RES),
        Box::new(B_GREEN_RES),
        Box::new(B_WHITE_RES),
    ];
    for (y, goods) in goods.iter().enumerate() {
        goods.iter().enumerate().for_each(|(i, &name)| {
            draw_res(
                &root,
                &res_color_map,
                name,
                res.get(name).unwrap_or(&0),
                30 + gap * (i as u32),
                350 + (y as u32) * 30,
            );
        });
    }

    // 当前时间
    let now: DateTime<Local> = Local::now();
    let time_str = now.format("%Y/%m/%d %H:%M:%S").to_string();
    draw_res_text(&root, &time_str, 780, 400, "#888");

    let shard = if target_shard == "all" {
        "all shard"
    } else {
        target_shard
    };
    let user = format!("{} {}", username, shard);
    draw_res_text(&root, &user, 780, 420, "#888");

    root.present()?;

    Ok(image_path.clone())
}
