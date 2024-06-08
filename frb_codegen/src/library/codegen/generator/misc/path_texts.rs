use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{TargetOrCommon, TargetOrCommonMap};
use crate::utils::basic_code::general_code::GeneralCode;
use crate::utils::file_utils::create_dir_all_and_write;
use itertools::Itertools;
use std::ops::Add;
use std::path::PathBuf;
use strum::IntoEnumIterator;

#[derive(Clone, Debug)]
pub(crate) struct PathTexts(pub Vec<PathText>);

impl Add for PathTexts {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0, rhs.0].concat())
    }
}

impl PathTexts {
    pub(crate) fn new_from_targets(
        path: &TargetOrCommonMap<PathBuf>,
        text: &Acc<Option<GeneralCode>>,
    ) -> Self {
        Self(
            TargetOrCommon::iter()
                .filter_map(|target| {
                    text[target]
                        .clone()
                        .map(|text_for_target| PathText::new(path[target].clone(), text_for_target))
                })
                .collect_vec(),
        )
    }

    pub(crate) fn merge(self) -> Self {
        Self(
            self.0
                .into_iter()
                .into_group_map_by(|x| x.path.clone())
                .into_iter()
                .map(|(path, items_of_same_path)| PathText {
                    path,
                    text: items_of_same_path
                        .into_iter()
                        .map(|x| x.text)
                        .reduce(|a, b| a + b)
                        .unwrap(),
                })
                .collect_vec(),
        )
    }

    pub(crate) fn write_to_disk(&self) -> anyhow::Result<()> {
        self.assert_no_duplicate_paths();
        for item in self.0.iter() {
            create_dir_all_and_write(&item.path, &item.text.all_code())?;
        }
        Ok(())
    }

    fn assert_no_duplicate_paths(&self) {
        let paths = self.paths();
        assert_eq!(
            paths.iter().unique().collect_vec().len(),
            paths.len(),
            "assert_no_duplicate_paths failed paths={paths:?}"
        );
    }

    pub(crate) fn paths(&self) -> Vec<PathBuf> {
        self.0.iter().map(|item| item.path.clone()).collect_vec()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct PathText {
    pub path: PathBuf,
    pub text: GeneralCode,
}

impl PathText {
    pub(crate) fn new(path: PathBuf, text: GeneralCode) -> Self {
        Self { path, text }
    }
}
