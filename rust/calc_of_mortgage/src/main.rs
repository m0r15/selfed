/**
 * by m0r15 (serenkvoav@gmail.com)
 */

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

/// Mortgage - ипотека
#[derive(Debug)]
struct Mortgage {
    /// сумма ипотеки
    sum:     f64, 
    /// процент по ипотеке (годовой)
    percent: f64, 
    /// период в месяцах
    period:  i32, 
}

impl Mortgage {

    /// заполняет структуру
    fn build(gage_sum: f64, gage_percent: f64, gage_period: i32) -> Mortgage {
        Mortgage {
            sum:     gage_sum,
            percent: gage_percent,
            period:  gage_period,
        }
    }

    /// Вычисление платежа по ипотеки в месяц
    fn pay_per_month(&self) -> f64 {
        let i: f64 = (self.percent / 100.0) / 12.0;

        // return result
        self.sum * ( i * (1.0 + i).powi(self.period) / ((1.0 + i).powi(self.period) - 1.0) )
    }

}

fn main() {
    let mortgage = Mortgage::build(4_200_000.0, 12.53, 160);

    println!("mortgage per month: {:.2}", mortgage.pay_per_month());
}
