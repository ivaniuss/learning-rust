use std::fmt;

#[derive(Debug, Clone)]
struct LendingPool {
    name: String,
    principal: f64,
    annual_rate: f64,
    compound_frequency: CompoundFrequency,
}

#[derive(Debug, Clone)]
enum CompoundFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

impl LendingPool {
    fn new(
        name: String,
        principal: f64,
        annual_rate: f64,
        compound_frequency: CompoundFrequency,
    ) -> Self {
        Self {
            name,
            principal,
            annual_rate,
            compound_frequency,
        }
    }

    fn calculate_balance(&self, years: f64) -> f64 {
        let n = self.compound_frequency.periods_per_year();
        let rate_per_period = self.annual_rate / n;
        let total_periods = n * years;

        // compound interest formula: A = P(1 + r/n)^(nt)
        self.principal * (1.0 + rate_per_period).powf(total_periods)
    }

    fn calculate_apy(&self) -> f64 {
        let n = self.compound_frequency.periods_per_year();
        let rate_per_period = self.annual_rate / n;

        // APY formula: (1 + r/n)^n - 1
        (1.0 + rate_per_period).powf(n) - 1.0
    }

    fn calculate_earnings(&self, years: f64) -> f64 {
        self.calculate_balance(years) - self.principal
    }

    fn simulate_dca(&self, monthly_deposit: f64, years: f64) -> f64 {
        let months = (years * 12.0) as i32;
        let monthly_rate = self.annual_rate / 12.0;
        let mut total_balance = self.principal;

        for month in 1..=months {
            total_balance *= 1.0 + monthly_rate;

            if month > 1 {
                total_balance += monthly_deposit;
            }
        }

        total_balance
    }

    fn compare_with(&self, other: &LendingPool, years: f64) -> PoolComparison {
        let self_balance = self.calculate_balance(years);
        let other_balance = other.calculate_balance(years);

        PoolComparison {
            pool1: self.clone(),
            pool2: other.clone(),
            pool1_balance: self_balance,
            pool2_balance: other_balance,
            difference: self_balance - other_balance,
            better_pool: if self_balance > other_balance {
                self.name.clone()
            } else {
                other.name.clone()
            },
        }
    }
}

#[derive(Debug)]
struct PoolComparison {
    pool1: LendingPool,
    pool2: LendingPool,
    pool1_balance: f64,
    pool2_balance: f64,
    difference: f64,
    better_pool: String,
}

impl fmt::Display for LendingPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pool: {} | Principal: ${:.2} | APR: {:.2}% | Frequency: {:?}", 
               self.name, self.principal, self.annual_rate * 100.0, self.compound_frequency)
    }
}

impl fmt::Display for PoolComparison {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "Comparison:\n{}: ${:.2}\n{}: ${:.2}\nDifference: ${:.2}\nBetter choice: {}", 
            self.pool1.name, self.pool1_balance,
            self.pool2.name, self.pool2_balance,
            self.difference.abs(), self.better_pool)
    }
}

impl CompoundFrequency {
    fn periods_per_year(&self) -> f64 {
        match self {
            CompoundFrequency::Daily => 365.0,
            CompoundFrequency::Weekly => 52.0,
            CompoundFrequency::Monthly => 12.0,
            CompoundFrequency::Quarterly => 4.0,
            CompoundFrequency::Annually => 1.0,
        }
    }
}

fn main() {
    println!("🚀 DeFi Lending Pool Calculator\n");
    
    let aave_pool = LendingPool::new(
        "Aave USDC".to_string(),
        10000.0,  // $10,000 principal
        0.08,     // 8% APR
        CompoundFrequency::Daily
    );
    
    let compound_pool = LendingPool::new(
        "Compound DAI".to_string(),
        10000.0,  // $10,000  principal
        0.075,    // 7.5% APR
        CompoundFrequency::Daily
    );
    
    let anchor_pool = LendingPool::new(
        "Anchor UST".to_string(),
        10000.0,  // $10,000 principal
        0.19,     // 19% APR
        CompoundFrequency::Weekly
    );
    
    println!("📊 Pools available:");
    println!("{}", aave_pool);
    println!("{}", compound_pool);
    println!("{}", anchor_pool);
    println!();
    
    println!("💰 APY Real (considering compound):");
    println!("{}: APR {:.2}% → APY {:.2}%", 
        aave_pool.name, aave_pool.annual_rate * 100.0, aave_pool.calculate_apy() * 100.0);
    println!("{}: APR {:.2}% → APY {:.2}%", 
        compound_pool.name, compound_pool.annual_rate * 100.0, compound_pool.calculate_apy() * 100.0);
    println!("{}: APR {:.2}% → APY {:.2}%", 
        anchor_pool.name, anchor_pool.annual_rate * 100.0, anchor_pool.calculate_apy() * 100.0);
    println!();
    
    let timeframes = vec![0.25, 0.5, 1.0, 2.0, 5.0];
    
    println!("📈 Balance projections:");
    println!("{:<15} {:<12} {:<12} {:<12} {:<12} {:<12}", 
        "Pool", "3 months", "6 months", "1 year", "2 years", "5 years");
    println!("{}", "-".repeat(80));
    
    for pool in [&aave_pool, &compound_pool, &anchor_pool] {
        print!("{:<15}", pool.name);
        for &years in &timeframes {
            let balance = pool.calculate_balance(years);
            print!(" ${:<11.0}", balance);
        }
        println!();
    }
    println!();
    
    println!("⚖️  Comparison after 1 year:");
    let comparison = aave_pool.compare_with(&compound_pool, 1.0);
    println!("{}", comparison);
    println!();
    
    println!("🔄 DCA simulation - Monthly deposit of $500:");
    let dca_balance = aave_pool.simulate_dca(500.0, 2.0);
    let total_invested = aave_pool.principal + (500.0 * 23.0); // 23 depósitos adicionales
    let dca_earnings = dca_balance - total_invested;
    
    println!("Total invested: ${:.2}", total_invested);
    println!("Final balance: ${:.2}", dca_balance);
    println!("Interest earnings: ${:.2}", dca_earnings);
    println!("ROI: {:.2}%", (dca_earnings / total_invested) * 100.0);
    
    println!("\n📚 DeFi concepts learned:");
    println!("• APR vs APY: APR is the nominal rate, APY includes the effect of compound");
    println!("• Compound Frequency: More frequent = greater real yield");
    println!("• DCA: Investment strategy to average prices");
    println!("• Pool Comparison: Key tool for yield farming");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_interest_calculation() {
        let pool = LendingPool::new(
            "Test Pool".to_string(),
            1000.0,
            0.10,
            CompoundFrequency::Yearly
        );
        
        let balance = pool.calculate_balance(1.0);
        assert!((balance - 1100.0).abs() < 0.01);
    }
    
    #[test]
    fn test_apy_calculation() {
        let pool = LendingPool::new(
            "Test Pool".to_string(),
            1000.0,
            0.12,
            CompoundFrequency::Monthly
        );
        
        let apy = pool.calculate_apy();
        // 12% APR compounded monthly should be ~12.68% APY
        assert!(apy > 0.126 && apy < 0.127);
    }
    
    #[test]
    fn test_pool_comparison() {
        let pool1 = LendingPool::new("Pool1".to_string(), 1000.0, 0.10, CompoundFrequency::Daily);
        let pool2 = LendingPool::new("Pool2".to_string(), 1000.0, 0.08, CompoundFrequency::Daily);
        
        let comparison = pool1.compare_with(&pool2, 1.0);
        assert!(comparison.difference > 0.0);
        assert_eq!(comparison.better_pool, "Pool1");
    }
}