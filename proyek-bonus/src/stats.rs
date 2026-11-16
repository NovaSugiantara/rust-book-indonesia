use crate::model::Link;
use serde::Serialize;

/// Ringkasan statistik yang dipinjam dari sebuah `Link`.
#[derive(Debug, Serialize)]
pub struct StatsView<'a> {
    pub code: &'a str,
    pub clicks: u64,
    pub label: String,
}

pub fn describe_clicks(clicks: u64) -> String {
    match clicks {
        count @ 0..=9 => format!("{count} klik, sepi"),
        count if count < 100 => format!("{count} klik, ramai"),
        count => format!("{count} klik, sangat ramai"),
    }
}

/// Fungsi ini membutuhkan lifetime eksplisit karena hasilnya meminjam `code`.
pub fn stats_for<'a>(link: &'a Link) -> StatsView<'a> {
    StatsView {
        code: link.code.as_str(),
        clicks: link.clicks,
        label: describe_clicks(link.clicks),
    }
}

pub fn sum_by<T, F>(items: &[T], score: F) -> u64
where
    F: Fn(&T) -> u64,
{
    items.iter().map(score).sum()
}

pub fn total_clicks(links: &[Link]) -> u64 {
    sum_by(links, |link| link.clicks)
}

pub fn summarize<'a>(links: &'a [Link]) -> Vec<StatsView<'a>> {
    links.iter().map(stats_for).collect()
}

pub fn top_links<'a>(links: &'a [Link], limit: usize) -> Vec<StatsView<'a>> {
    let mut sorted: Vec<&Link> = links.iter().collect();
    sorted.sort_by_key(|link| std::cmp::Reverse(link.clicks));
    sorted.into_iter().take(limit).map(stats_for).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shortcode::ShortCode;

    fn link(code: &str, clicks: u64) -> Link {
        let mut link = Link::new(
            ShortCode::parse(code).unwrap(),
            "https://example.com".to_string(),
        );
        link.clicks = clicks;
        link
    }

    #[test]
    fn totals_clicks_with_closure() {
        let links = vec![link("satu", 2), link("dua2", 3), link("tiga", 5)];
        assert_eq!(total_clicks(&links), 10);
    }

    #[test]
    fn sorts_by_most_clicked() {
        let links = vec![link("satu", 2), link("dua2", 9), link("tiga", 5)];
        let top = top_links(&links, 2);
        assert_eq!(top[0].code, "dua2");
        assert_eq!(top[1].code, "tiga");
    }

    #[test]
    fn labels_click_buckets() {
        assert_eq!(describe_clicks(0), "0 klik, sepi");
        assert_eq!(describe_clicks(10), "10 klik, ramai");
        assert!(describe_clicks(1_000).contains("sangat ramai"));
    }
}
