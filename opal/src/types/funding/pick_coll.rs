use crate::{traits::filter_by_coll::FilterByColl, CollResult, FundingColl, PickCollConfig};

#[derive(Clone, Debug, Default)]
pub struct PickColl {
    pub config: PickCollConfig,
    pub colls: Vec<CollResult>,
}

impl PickColl {
    fn filter_all(&self, coll: &FundingColl) -> bool {
        let area = self.config.area.clone();
        let target = self.config.target_config.clone();
        target.my_label.filter_by_coll(coll)
            || (target.my_favorite.select && area.favorite.filter_by_coll(coll))
            || (self.config.area.block.filter_by_coll(coll))
    }
    pub fn run(&self) -> Vec<FundingColl> {
        println!("PickColl::run()");
        self.colls
            .iter()
            .filter_map(|x| {
                let labels = self.config.area.label.get_from_coll_name(&x.slug);
                let f_coll = FundingColl {
                    db: x.clone(),
                    labels,
                };
                if self.filter_all(&f_coll) {
                    Some(f_coll)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
