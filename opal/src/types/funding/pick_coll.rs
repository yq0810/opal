use std::collections::HashMap;

use log::debug;

use crate::{
    func_components::strategy_input, traits::filter_by_coll::FilterByColl, CollResult,
    FloorPriceResult, FundingColl, PickCollConfig,
};

type Slug = String;
#[derive(Clone, Debug, Default)]
pub struct PickColl {
    pub config: PickCollConfig,
    pub colls: Vec<CollResult>,
    pub fps_map: HashMap<Slug, FloorPriceResult>,
}

impl PickColl {
    fn filter_area_all(&self, coll: &FundingColl) -> bool {
        let area = self.config.area.clone();
        let target = self.config.target_config.clone();
        let result = target.full.select || target.my_label.filter_by_coll(coll) || {
            if target.my_favorite.select {
                area.favorite.filter_by_coll(coll)
            } else {
                true
            }
        } && ((if target
            .verify_opensea
            .select
        {
            coll.db.is_verified == 1
        } else {
            true
        })
            && (if target.verify_twitter.select {
                coll.db.twitter_is_verified == 1
            } else {
                true
            }));
        debug!("{:?}", coll.db);
        result && (self.config.area.block.filter_by_coll(coll))
    }
    fn filter_strategy_all(&self, coll: &FundingColl) -> bool {
        let strategy = self.config.strategy.clone();
        let coll_total_volume_value = coll.fp_last.as_ref().map(|x| x.total_volume);
        let strategy_total_volume_value = strategy.total_volume.volume_total_value;
        let result = (strategy.total_volume.volume_total_select
            && coll_total_volume_value > Some(strategy_total_volume_value));
        strategy.total_volume.volume_total_select == false || result
    }
    pub fn run(&self) -> (Vec<FundingColl>, Vec<FundingColl>) {
        println!("PickColl::run()");
        let mut picked_colls = vec![];
        let mut excepted_colls = vec![];
        let _ = self
            .colls
            .iter()
            .map(|x| {
                let labels = self.config.area.label.get_from_coll_name(&x.slug);
                let f_coll = FundingColl {
                    db: x.clone(),
                    labels,
                    fp_last: self.fps_map.get(&x.slug).map(|x| x.clone()),
                };
                if self.filter_area_all(&f_coll) && self.filter_strategy_all(&f_coll) {
                    picked_colls.push(f_coll);
                } else {
                    excepted_colls.push(f_coll);
                }
            })
            .collect::<Vec<_>>();
        (picked_colls, excepted_colls)
    }
}
