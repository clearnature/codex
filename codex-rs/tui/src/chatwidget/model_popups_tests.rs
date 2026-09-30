use super::*;
use pretty_assertions::assert_eq;

/// §3.6 碎片拼句的中文侧承重断言：`current()=Zh` 时「More reasoning…」的
/// description（档位名 + 连接词 + 模板整句）里不能残留任何 ASCII 单词——
/// 档位标签与连接词必须与模板一起接入 `tr`，否则得到半中半英句。
#[test]
fn zh_advanced_usage_description_contains_no_ascii_words() {
    let cases: [&[ReasoningEffortConfig]; 3] = [
        &[ReasoningEffortConfig::Max],
        &[ReasoningEffortConfig::Ultra],
        &[ReasoningEffortConfig::Max, ReasoningEffortConfig::Ultra],
    ];
    for efforts in cases {
        let zh = ChatWidget::advanced_usage_description(Lang::Zh, efforts);
        assert!(
            !zh.chars().any(|c| c.is_ascii_alphabetic()),
            "expected a fully localized zh description with no ASCII words, got {zh:?}"
        );
    }
}

/// H1 的英文侧：`Lang::En` 下该 description 必须逐字节等于重构前的英文
/// 原文（单复数两支 + 连接词），这是快照零漂移在本函数上的直接依据。
#[test]
fn en_advanced_usage_description_stays_byte_identical() {
    assert_eq!(
        ChatWidget::advanced_usage_description(Lang::En, &[ReasoningEffortConfig::Max]),
        "Max consumes usage limits faster"
    );
    assert_eq!(
        ChatWidget::advanced_usage_description(
            Lang::En,
            &[ReasoningEffortConfig::Max, ReasoningEffortConfig::Ultra],
        ),
        "Max and Ultra consume usage limits faster"
    );
}
