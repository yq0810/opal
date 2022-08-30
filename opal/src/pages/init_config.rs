use std::collections::{HashMap, HashSet};

use multimap::MultiMap;

use crate::{components::setting_card, BlockSetting, FavoriteSetting, LabelText};

use super::Config;

pub fn index_config() -> Config {
    let mut config = Config::default();
    config.setting_card.strategy.s_one.volume_rate_value = 30;
    config
        .setting_card
        .strategy
        .total_volume
        .volume_total_select = true;
    config.setting_card.strategy.total_volume.volume_total_value = 29387.0;

    config.area.label.current = {
        let mut init_area: MultiMap<LabelText, String> = MultiMap::new();
        init_area.insert("test".into(), "azuki".to_string());
        init_area.insert("good".into(), "azuki".to_string());
        init_area.insert("good".into(), "boredapeyachtclub".to_string());
        init_area
    };

    config.area.favorite.current = {
        let mut init_area: HashMap<String, FavoriteSetting> = HashMap::new();
        init_area.insert(
            "doodles-official".into(),
            FavoriteSetting {
                slug: "doodles-official".to_string(),
                bool: true,
            },
        );
        init_area
    };

    config.area.block.current = {
        let mut init_area: HashMap<String, BlockSetting> = HashMap::new();
        init_area.insert(
            "otherdeed".into(),
            BlockSetting {
                slug: "otherdeed".to_string(),
                bool: true,
            },
        );
        init_area
    };
    config.setting_card = {
        let mut init_setting = config.setting_card.clone();
        init_setting.active_tab = 0;
        init_setting
    };

    config.setting_card.target.my_label.total_labels = {
        let labels = config
            .area
            .label
            .current
            .iter()
            .map(|(x, _)| x.clone())
            .collect::<HashSet<_>>();
        labels
    };

    config.setting_card.target.my_label.selected_labels = {
        let labels = config
            .setting_card
            .target
            .my_label
            .total_labels
            .iter()
            .filter(|x| x.0 == "good".to_string())
            .map(|(x)| x.clone())
            .collect::<HashSet<_>>();
        labels
    };
    config.setting_card.target = {
        let mut config = config.setting_card.target;
        config.my_label.select = true;
        config.my_favorite.select = true;
        config.verify_twitter.select = true;
        config.verify_opensea.select = true;
        config.full.select = true;
        config
    };

    config.setting_card.funding_rule = {
        let mut init_funding = config.setting_card.funding_rule.clone();
        init_funding.total_amount_limit.active = true;
        init_funding.total_amount_limit.value = 10;

        init_funding.total_tx_count_limit.active = true;
        init_funding.total_tx_count_limit.value = 2;

        init_funding.unit_price_limit.active = true;
        init_funding.unit_price_limit.value = 2;

        init_funding
    };

    config
}
