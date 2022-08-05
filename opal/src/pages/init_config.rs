use multimap::MultiMap;

use crate::LabelText;

use super::Config;

pub fn index_config() -> Config {
    let mut config = Config::default();
    config.setting_card.strategy.s_one.volume_rate_value = 30;
    config.setting_card.strategy.s_two.volume_total_value = 12500.0;
    config.setting_card.strategy.s_two.volume_total_value = 12500.0;

    config.area.label.current = {
        let mut init_area: MultiMap<LabelText, String> = MultiMap::new();
        init_area.insert("testLabel".into(), "azuki".to_string());
        init_area
    };

    config.setting_card = {
        let mut init_setting = config.setting_card.clone();
        init_setting.active_tab = 1;
        init_setting
    };
    config
}
