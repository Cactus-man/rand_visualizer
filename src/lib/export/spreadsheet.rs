use fraction::Fraction;
use xlsxwriter::Workbook;

use super::{Export, Report};

pub struct Excel(Workbook);

impl Excel {
    pub fn new(filename: &str) -> Self {
        let filename = format!("{}.xlsx", filename);
        Self(Workbook::new(&filename))
    }

    fn to_sheet<'s>(self, report: &'s Report<'s>) -> anyhow::Result<()> {
        let workbook = self.0;
        let mut sheet = workbook.add_worksheet(Some(report.name))?;

        sheet.write_string(0, 0, "Augenzahl", None)?;
        sheet.write_string(0, 1, "Häufigkeit", None)?;
        sheet.write_string(0, 2, "Relative H.", None)?;
        sheet.write_string(0, 3, "in %", None)?;

        for (i, (dots, occurences, percent)) in report.with_percentages().into_iter().enumerate() {
            {
                let row: u32 = (i + 1).try_into()?;

                sheet.write_number(row, 0, dots.try_into()?, None)?;
                sheet.write_number(row, 1, occurences as f64, None)?;

                let rel = Fraction::new(occurences as u64, report.total as u64);
                sheet.write_string(row, 2, &rel.to_string(), None)?;
                sheet.write_number(row, 3, percent, None)?;
            }
        }

        sheet.set_column(0, 5, 30., None)?;

        Ok(())
    }
}

impl Export for Excel {
    fn export<'s>(self, report: &'s Report<'s>) {
        // TODO: Error handling
        self.to_sheet(report).unwrap();
    }
}
