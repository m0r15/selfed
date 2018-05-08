/*
 * Калькулятор для ипотеки
 * 
 * Вычислить месячные выплаты фиксированного срока в течении 
 * заданных N сроков с заданной процентной ставкой.
 * 
 * Инуитентная схема расчета
 * 
 * x = S * p / (1 - (1 + P) ^ (1-M))
 * x - ежемесячные выплаты
 * S - сумма ипотеки
 * p - (1/12) годового %
 * M - общий период кредитования в месяцахы
 */

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

#[derive(Debug)]
struct Mortgage {
    sum: f64,
    percent: f64,
    period: i32,
}

impl Mortgage {

    fn new() -> Mortgage {
        Mortgage {
            sum: 0.0,
            percent: 0.0,
            period: 0,
        }
    }

    fn build(gage_sum: f64, gage_percent: f64, gage_period: i32) -> Mortgage {
        Mortgage {
            sum: gage_sum,
            percent: gage_percent,
            period: gage_period,
        }
    }

    fn pay_per_month(&self) -> f64 {
        let p: f64 = self.percent / 12.0;

        self.sum * p / (1.0 - (1.0 + p).powi(1 - self.period)) // result
    }

}

fn main() {
    let mortgage = Mortgage::build(12.53, 4_200_000.0, 160);

    println!("{:.2}", mortgage.pay_per_month());
}
