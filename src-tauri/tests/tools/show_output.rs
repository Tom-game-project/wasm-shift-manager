use shift_manager_tauri_lib::application::dto::MonthlyShiftResult;

pub fn show_monthly_shift_result_debug_data(monthly_shift_result: &MonthlyShiftResult) {
    println!("\n=======================================================");
    println!(
        "🗓️ [DEBUG] シフト出力結果 (計 {} 週間)",
        monthly_shift_result.weeks.len()
    );
    println!("=======================================================");

    // 曜日の表示用ラベル (0=Mon ~ 6=Sun に対応)
    let day_labels = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

    for (week_idx, week_info) in monthly_shift_result.weeks.iter().enumerate() {
        println!(
            "📅 [Week {}] ------------------------------------------",
            week_idx + 1
        );

        match week_info.status.as_str() {
            "Active" => {
                let Some(week) = week_info.shift.as_ref() else {
                    println!("   (Active but no shift data)");
                    continue;
                };

                for (day_idx, day) in week.days.iter().enumerate() {
                    let label = day_labels.get(day_idx).unwrap_or(&"???");

                    // 名前のリストをカンマ区切りの文字列にする。空なら "(なし)" と表示
                    let morning_str = if day.morning.is_empty() {
                        "(なし)".to_string()
                    } else {
                        day.morning.join(", ")
                    };

                    let afternoon_str = if day.afternoon.is_empty() {
                        "(なし)".to_string()
                    } else {
                        day.afternoon.join(", ")
                    };

                    println!(
                        "   {} : [午前] {:<15} | [午後] {}",
                        label, morning_str, afternoon_str
                    );
                }
            }
            "Skipped" => {
                println!("   (Skipped)");
            }
            other => {
                println!("   ({other})");
            }
        }
    }
    println!("=======================================================\n");
}

use shift_manager_tauri_lib::domain::rule_model::PlanConfig;

pub fn show_plan_config_debug_data(config: &PlanConfig) {
    println!("\n=======================================================");
    println!("📋 [DEBUG] ルール設定データ (Plan ID: {})", config.plan.id);
    println!("=======================================================");
    for group in &config.groups {
        println!("👥 グループ: {} (ID: {})", group.group.name, group.group.id);
        for (i, member) in group.members.iter().enumerate() {
            println!("   ┣ メンバー[{}]: {} (ID: {})", i, member.name, member.id);
        }
    }
    println!("-------------------------------------------------------");
    for rule in &config.rules {
        println!("📅 ルール: {} (ID: {})", rule.rule.name, rule.rule.id);
        for assign in &rule.assignments {
            println!(
                "   ┣ アサイン: 曜日[{:?}] 時間[{:?}] -> グループID[{}]のメンバー[{}]",
                assign.weekday,
                assign.shift_time_type,
                assign.target_group_id,
                assign.target_member_index
            );
        }
    }
    println!("=======================================================\n");
}
