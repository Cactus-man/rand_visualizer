use super::{Export, Report};
use term_table::table_cell::{Alignment, TableCell};
use term_table::TableStyle;
use term_table::{row::Row, Table};

pub struct Console;

impl Export for Console {
    fn export<'s>(self, report: &'s Report<'s>) {
        let title = format!("{}: {} Würfe", report.name, report.total);
        let title = Row::new([TableCell::new_with_alignment(title, 4, Alignment::Center)]);
        let header = Row::new(["#", "Anzahl", "Relative H."]);

        let mut table = Table::with_rows(vec![title, header]);
        table.style = TableStyle::blank();

        let data = report.with_percentages();
        let &max = data.iter().map(|(_, o, _)| o).max().unwrap_or(&0);
        for (d, n, p) in data {
            let row = Row::new([
                d.to_string(),
                n.to_string(),
                format!("{:.2}%", p),
                "#".repeat(n * 20 / max),
            ]);
            table.add_row(row);
        }

        eprintln!("{}", table.to_string());
    }
}
