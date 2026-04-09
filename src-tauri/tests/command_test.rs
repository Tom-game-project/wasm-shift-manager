mod tools;

#[cfg(test)]
mod command_tests {
    use shift_manager_tauri_lib::application::time::{calculate_abs_week, calculate_weeks_in_month};
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;
    use tauri::Manager;

    use shift_manager_tauri_lib::{
        // domain::{
        //     shift_calendar_model::{WeekStatus},
        // },
        // // RuleRepository をインポート
        // infrastructure::{
        //     calendar_repo::CalendarRepository,
        //     rule_repo::RuleRepository,
        // },
        application::commands::*,
        AppServices,
    };

    use crate::tools;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create memory pool");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        pool
    }

    async fn setup_test_services() -> AppServices {
        let pool = setup_test_db().await;

        AppServices::new(pool)
    }

    #[tokio::test]
    async fn test_full_scenario_from_ui() {
        // 1. テスト用サービスと Tauri モックアプリの起動
        let services = setup_test_services().await;
        let app = tauri::test::mock_builder()
            .manage(services)
            .build(tauri::generate_context!())
            .unwrap();
        let state = app.state::<AppServices>();

        // 2. [コマンド実行] プランの作成
        let plan_name = "2026年 シフト計画".to_string();
        let plan_id = create_new_plan(plan_name, state.clone()).await.unwrap();
        assert!(plan_id > 0);

        // 3. [コマンド実行] スタッフグループとメンバーの作成
        let group_id = add_staff_group(plan_id, "正社員".to_string(), state.clone())
            .await
            .unwrap();
        let member1_id = add_staff_member(group_id, "田中".to_string(), state.clone())
            .await
            .unwrap();
        let member2_id = add_staff_member(group_id, "佐藤".to_string(), state.clone())
            .await
            .unwrap();

        // 4. [コマンド実行] ルールとアサインの作成
        let rule_id = add_weekly_rule(plan_id, "標準ルール".to_string(), state.clone())
            .await
            .unwrap();
        // 月曜日(1) の 午前(0) に グループ(group_id) の 0番目の人 をアサイン
        let _assign_id = add_rule_assignment(rule_id, 0, 0, group_id, 0, state.clone())
            .await
            .unwrap();

        // 5. 設定が正しく保存されたか確認
        let config = get_plan_config(plan_id, state.clone()).await.unwrap();

        assert_eq!(config.groups[0].members.len(), 2);
        assert_eq!(config.rules[0].assignments.len(), 1);

        // ★ 修正：先ほど作った member1_id と member2_id が正しく保存されているかを厳密にテスト
        let members = &config.groups[0].members;
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].id, member1_id); // 0番目が「田中さん」であること
        assert_eq!(members[0].name, "田中");
        assert_eq!(members[1].id, member2_id); // 1番目が「佐藤さん」であること

        // アサインの検証：0番目（田中さん）がアサインされていること
        let assignment = &config.rules[0].assignments[0];
        assert_eq!(assignment.target_group_id, group_id);
        assert_eq!(assignment.target_member_index, 0); // 田中さんのインデックス

        // 6. [コマンド実行] カレンダー作成とタイムラインの追記
        let abs_week = calculate_abs_week(2026, 0, 1).unwrap();

        create_calendar(plan_id, abs_week, 0, state.clone())
            .await
            .unwrap();

        // 対象月の先頭週から 3週間分のルールをセット (Active, Active, Skipped)
        let timeline_data = vec![Some(rule_id), Some(rule_id), None];
        append_timeline(plan_id, abs_week, timeline_data, state.clone())
            .await
            .unwrap();

        // 7. [コマンド実行] シフトの導出テスト (現状はダミーデータが返るか確認)
        // 2026年 1月のシフトをリクエスト
        let monthly_shift = derive_monthly_shift(plan_id, 2026, 0, state.clone())
            .await
            .unwrap();

        // 検証: 月表示の週数はカレンダー計算と一致すること
        assert_eq!(
            monthly_shift.weeks.len(),
            calculate_weeks_in_month(2026, 0) as usize
        );
        assert_eq!(
            monthly_shift
                .weeks
                .iter()
                .map(|week| week.status.as_str())
                .collect::<Vec<_>>(),
            vec!["Active", "Active", "Skipped", "Pending", "Pending"]
        );
        let first_week = &monthly_shift.weeks[0];
        assert_eq!(first_week.status, "Active");
        let first_week_shift = first_week
            .shift
            .as_ref()
            .expect("active week should include shift data");
        assert_eq!(first_week_shift.days.len(), 7);
        assert_eq!(first_week_shift.days[0].morning, vec!["田中".to_string()]);
        assert!(first_week_shift.days[0].afternoon.is_empty());

        let second_week_shift = monthly_shift.weeks[1]
            .shift
            .as_ref()
            .expect("second active week should include shift data");
        assert_eq!(second_week_shift.days[0].morning, vec!["佐藤".to_string()]);
    }

    #[tokio::test]
    async fn test_full_scenario_from_ui2() {
        let services = setup_test_services().await;
        let app = tauri::test::mock_builder()
            .manage(services)
            .build(tauri::generate_context!())
            .unwrap();
        let state = app.state::<AppServices>();

        // 1〜4. データ作成 (省略・前回のコードと同じ)
        let plan_name = "2026年 シフト計画".to_string();
        let plan_id = create_new_plan(plan_name, state.clone()).await.unwrap();
        let group_id = add_staff_group(plan_id, "正社員".to_string(), state.clone())
            .await
            .unwrap();
        let member1_id = add_staff_member(group_id, "田中".to_string(), state.clone())
            .await
            .unwrap();
        let _member2_id = add_staff_member(group_id, "佐藤".to_string(), state.clone())
            .await
            .unwrap();
        let rule_id = add_weekly_rule(plan_id, "標準ルール".to_string(), state.clone())
            .await
            .unwrap();
        let _assign_id = add_rule_assignment(rule_id, 1, 0, group_id, 0, state.clone())
            .await
            .unwrap();

        // =================================================================
        // ★ 追加1：ルール設定のわかりやすいデバッグ表示
        // =================================================================
        let config = get_plan_config(plan_id, state.clone()).await.unwrap();

        tools::show_output::show_plan_config_debug_data(&config);

        // アサーション (前回のまま)
        assert_eq!(config.groups[0].members[0].id, member1_id);

        let abs_week = calculate_abs_week(2026, 0, 1).unwrap();

        // 6. カレンダー作成とタイムライン追記 (前回のまま)
        create_calendar(plan_id, abs_week, 0, state.clone())
            .await
            .unwrap();
        let timeline_data = vec![Some(rule_id), Some(rule_id), None];
        append_timeline(plan_id, abs_week, timeline_data, state.clone())
            .await
            .unwrap();

        // =================================================================
        // ★ 追加2：タイムラインのデバッグ表示（Repositoryのメソッドを直接呼ぶ）
        // =================================================================
        let _ = state.calendar.debug_print_timeline(plan_id).await;

        // 7. シフト導出テスト (前回のまま)
        let monthly_shift = derive_monthly_shift(plan_id, 2026, 0, state.clone())
            .await
            .unwrap();

        tools::show_output::show_monthly_shift_result_debug_data(&monthly_shift);

        assert_eq!(
            monthly_shift.weeks.len(),
            calculate_weeks_in_month(2026, 0) as usize
        );
        assert_eq!(
            monthly_shift
                .weeks
                .iter()
                .map(|week| week.status.as_str())
                .collect::<Vec<_>>(),
            vec!["Active", "Active", "Skipped", "Pending", "Pending"]
        );

        let first_week_shift = monthly_shift.weeks[0]
            .shift
            .as_ref()
            .expect("first active week should include shift data");
        assert_eq!(first_week_shift.days[1].morning, vec!["田中".to_string()]);

        let second_week_shift = monthly_shift.weeks[1]
            .shift
            .as_ref()
            .expect("second active week should include shift data");
        assert_eq!(second_week_shift.days[1].morning, vec!["佐藤".to_string()]);
    }
}
