//! Plain-text + HTML bodies for every notification category. Pure functions:
//! the caller gathers the numbers, these only format them. The HTML is a
//! single inline-styled column with no images, links, fonts or tracking.
//!
//! Signatures (all return `EmailMessage` with `html: Some(..)`):
//! ```ignore
//! templates::reminder(date_label: &str)                       // "Tuesday 21 October"
//! templates::progress(awards: &[AwardLine])                   // bundled list; one item is fine
//! templates::weekly_summary(summary: &WeeklySummary)
//! templates::streak_alert(streak_days: u32)
//! templates::pace_alert(habit_name: &str, unit: &str, today_quantity: f64, average_quantity: f64)
//! templates::test_email()
//! ```
//! `AwardLine { title, points }` and `WeeklySummary { week_label,
//! net_minutes_this_week, net_minutes_last_week, points_this_week,
//! clean_streak_days, most_improved: Option<ImprovedHabit { name, detail }> }`
//! are plain structs defined below. Net minutes are signed: positive means
//! life gained, negative life lost.

use super::EmailMessage;

const FOOTER: &str =
    "Sent by Ripple from your computer. Change or turn off these emails in Settings → Notifications.";

/// One newly earned award for the bundled progress email.
#[derive(Debug, Clone, PartialEq)]
pub struct AwardLine {
    /// e.g. "Reached level 3 — Stream" or "First week logged".
    pub title: String,
    pub points: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImprovedHabit {
    pub name: String,
    /// Caller-formatted change, e.g. "4 fewer cigarettes than last week".
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeeklySummary {
    /// e.g. "13–19 October".
    pub week_label: String,
    pub net_minutes_this_week: f64,
    pub net_minutes_last_week: f64,
    pub points_this_week: i64,
    pub clean_streak_days: u32,
    pub most_improved: Option<ImprovedHabit>,
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Wraps already-escaped body HTML in the shared shell.
fn page(title: &str, body_html: &str) -> String {
    format!(
        "<!doctype html><html><body style=\"margin:0;padding:24px;background:#f4f6f8;\">\
<div style=\"max-width:520px;margin:0 auto;background:#ffffff;border-radius:8px;padding:24px;\
font-family:-apple-system,'Segoe UI',Helvetica,Arial,sans-serif;font-size:15px;line-height:1.5;color:#1f2933;\">\
<h1 style=\"margin:0 0 12px;font-size:20px;color:#0b4f6c;\">{}</h1>{}\
<p style=\"margin:24px 0 0;font-size:12px;color:#7b8794;\">{}</p></div></body></html>",
        escape(title),
        body_html,
        escape(FOOTER)
    )
}

fn para(text: &str) -> String {
    format!("<p style=\"margin:0 0 12px;\">{}</p>", escape(text))
}

fn text_body(lines: &[String]) -> String {
    format!("{}\n\n--\n{FOOTER}\n", lines.join("\n"))
}

fn message(subject: String, text_lines: &[String], title: &str, body_html: String) -> EmailMessage {
    EmailMessage {
        subject,
        text: text_body(text_lines),
        html: Some(page(title, &body_html)),
    }
}

/// "+1 h 5 min", "−20 min", "0 min".
pub fn format_minutes(minutes: f64) -> String {
    let rounded = minutes.round() as i64;
    let sign = match rounded {
        n if n > 0 => "+",
        n if n < 0 => "−",
        _ => "",
    };
    let abs = rounded.unsigned_abs();
    let (h, m) = (abs / 60, abs % 60);
    let magnitude = match (h, m) {
        (0, m) => format!("{m} min"),
        (h, 0) => format!("{h} h"),
        (h, m) => format!("{h} h {m} min"),
    };
    format!("{sign}{magnitude}")
}

fn format_quantity(q: f64) -> String {
    if (q - q.round()).abs() < 0.05 {
        format!("{}", q.round() as i64)
    } else {
        format!("{q:.1}")
    }
}

fn plural(n: u32, one: &str, many: &str) -> String {
    format!("{n} {}", if n == 1 { one } else { many })
}

pub fn reminder(date_label: &str) -> EmailMessage {
    let line = "Nothing is logged for today yet. A quick entry keeps your timeline honest.";
    message(
        "Time to log today's habits".to_string(),
        &[format!("{date_label}"), String::new(), line.to_string()],
        "Time to log today's habits",
        format!("{}{}", para(date_label), para(line)),
    )
}

pub fn progress(awards: &[AwardLine]) -> EmailMessage {
    let total: i64 = awards.iter().map(|a| a.points).sum();
    let subject = match awards {
        [one] => format!("New in Ripple: {}", one.title),
        _ => format!("{} new achievements in Ripple", awards.len()),
    };
    let mut text = vec!["You've earned:".to_string()];
    text.extend(awards.iter().map(|a| format!("• {} (+{} pts)", a.title, a.points)));
    text.push(String::new());
    text.push(format!("Total: +{total} points"));
    let items: String = awards
        .iter()
        .map(|a| {
            format!(
                "<li style=\"margin:0 0 6px;\">{} <span style=\"color:#2f855a;\">+{}&nbsp;pts</span></li>",
                escape(&a.title),
                a.points
            )
        })
        .collect();
    let html = format!(
        "{}<ul style=\"margin:0 0 12px;padding-left:20px;\">{items}</ul>{}",
        para("You've earned:"),
        para(&format!("Total: +{total} points"))
    );
    message(subject, &text, "Nice progress", html)
}

pub fn weekly_summary(s: &WeeklySummary) -> EmailMessage {
    let rows: Vec<(String, String)> = vec![
        ("Net life, this week".to_string(), format_minutes(s.net_minutes_this_week)),
        ("Net life, last week".to_string(), format_minutes(s.net_minutes_last_week)),
        ("Points earned".to_string(), format!("{}", s.points_this_week)),
        ("Clean streak".to_string(), plural(s.clean_streak_days, "day", "days")),
    ];
    let mut text = vec![s.week_label.clone(), String::new()];
    text.extend(rows.iter().map(|(k, v)| format!("{k}: {v}")));
    if let Some(h) = &s.most_improved {
        text.push(format!("Most improved: {} — {}", h.name, h.detail));
    }
    let mut table: String = rows
        .iter()
        .map(|(k, v)| {
            format!(
                "<tr><td style=\"padding:6px 0;color:#52606d;\">{}</td>\
<td style=\"padding:6px 0;text-align:right;font-weight:600;\">{}</td></tr>",
                escape(k),
                escape(v)
            )
        })
        .collect();
    if let Some(h) = &s.most_improved {
        table.push_str(&format!(
            "<tr><td style=\"padding:6px 0;color:#52606d;\">Most improved</td>\
<td style=\"padding:6px 0;text-align:right;font-weight:600;\">{}<br><span style=\"font-weight:400;color:#52606d;\">{}</span></td></tr>",
            escape(&h.name),
            escape(&h.detail)
        ));
    }
    let html = format!(
        "{}<table style=\"width:100%;border-collapse:collapse;\">{table}</table>",
        para(&s.week_label)
    );
    message(
        format!("Your week in Ripple: {}", s.week_label),
        &text,
        "Your week in Ripple",
        html,
    )
}

pub fn streak_alert(streak_days: u32) -> EmailMessage {
    let line = format!("You're on a {streak_days}-day clean streak — keep today clean.");
    message(
        format!("{streak_days}-day clean streak"),
        &[line.clone()],
        "Keep the streak going",
        para(&line),
    )
}

pub fn pace_alert(habit_name: &str, unit: &str, today_quantity: f64, average_quantity: f64) -> EmailMessage {
    let line = format!(
        "{habit_name} is ahead of your usual pace today: {} {unit} so far, against a daily average of {} {unit} over the last four weeks.",
        format_quantity(today_quantity),
        format_quantity(average_quantity)
    );
    message(
        format!("{habit_name} is ahead of pace today"),
        &[line.clone()],
        "Heads up on today's pace",
        para(&line),
    )
}

pub fn test_email() -> EmailMessage {
    let line = "Email notifications from Ripple are working.";
    message(
        "Ripple test email".to_string(),
        &[line.to_string()],
        "It works",
        para(line),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html(m: &EmailMessage) -> &str {
        m.html.as_deref().unwrap()
    }

    #[test]
    fn every_template_has_text_html_and_no_external_resources() {
        let all = [
            reminder("Tuesday 21 October"),
            progress(&[AwardLine { title: "Level 2".to_string(), points: 50 }]),
            weekly_summary(&WeeklySummary {
                week_label: "13–19 October".to_string(),
                net_minutes_this_week: 30.0,
                net_minutes_last_week: -45.0,
                points_this_week: 120,
                clean_streak_days: 4,
                most_improved: None,
            }),
            streak_alert(3),
            pace_alert("Coffee", "cups", 4.0, 2.0),
            test_email(),
        ];
        for m in &all {
            assert!(!m.subject.is_empty());
            assert!(m.text.contains(FOOTER));
            let h = html(m);
            assert!(h.starts_with("<!doctype html>"));
            for banned in ["<img", "src=", "href=", "<link", "<script", "url("] {
                assert!(!h.contains(banned), "{banned} in {}", m.subject);
            }
        }
    }

    #[test]
    fn reminder_names_the_day() {
        let m = reminder("Tuesday 21 October");
        assert_eq!(m.subject, "Time to log today's habits");
        assert!(m.text.starts_with("Tuesday 21 October\n"));
    }

    #[test]
    fn progress_bundles_every_award_with_a_total() {
        let m = progress(&[
            AwardLine { title: "Reached level 3 — Stream".to_string(), points: 100 },
            AwardLine { title: "First week logged".to_string(), points: 25 },
        ]);
        assert_eq!(m.subject, "2 new achievements in Ripple");
        assert!(m.text.contains("• Reached level 3 — Stream (+100 pts)"));
        assert!(m.text.contains("• First week logged (+25 pts)"));
        assert!(m.text.contains("Total: +125 points"));
        assert_eq!(html(&m).matches("<li").count(), 2);

        let single = progress(&[AwardLine { title: "Level 2".to_string(), points: 50 }]);
        assert_eq!(single.subject, "New in Ripple: Level 2");
    }

    #[test]
    fn weekly_summary_compares_weeks_and_names_most_improved() {
        let m = weekly_summary(&WeeklySummary {
            week_label: "13–19 October".to_string(),
            net_minutes_this_week: 95.0,
            net_minutes_last_week: -20.0,
            points_this_week: 140,
            clean_streak_days: 1,
            most_improved: Some(ImprovedHabit {
                name: "Cigarettes".to_string(),
                detail: "12 fewer than last week".to_string(),
            }),
        });
        assert_eq!(m.subject, "Your week in Ripple: 13–19 October");
        assert!(m.text.contains("Net life, this week: +1 h 35 min"));
        assert!(m.text.contains("Net life, last week: −20 min"));
        assert!(m.text.contains("Points earned: 140"));
        assert!(m.text.contains("Clean streak: 1 day"));
        assert!(m.text.contains("Most improved: Cigarettes — 12 fewer than last week"));
        assert!(html(&m).contains("Cigarettes"));
    }

    #[test]
    fn streak_and_pace_wording() {
        assert!(streak_alert(5)
            .text
            .starts_with("You're on a 5-day clean streak — keep today clean."));
        let m = pace_alert("Coffee", "cups", 4.0, 2.25);
        assert_eq!(m.subject, "Coffee is ahead of pace today");
        assert!(m.text.contains("4 cups so far, against a daily average of 2.2 cups"));
    }

    #[test]
    fn user_text_is_escaped_in_html() {
        let m = pace_alert("<b>Tea & \"Biscuits\"</b>", "cups", 3.0, 1.0);
        let h = html(&m);
        assert!(h.contains("&lt;b&gt;Tea &amp; &quot;Biscuits&quot;&lt;/b&gt;"));
        assert!(!h.contains("<b>Tea"));
    }

    #[test]
    fn minutes_format_with_sign_and_hours() {
        assert_eq!(format_minutes(0.2), "0 min");
        assert_eq!(format_minutes(45.0), "+45 min");
        assert_eq!(format_minutes(-120.0), "−2 h");
        assert_eq!(format_minutes(61.0), "+1 h 1 min");
    }
}
