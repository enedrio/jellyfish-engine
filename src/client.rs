use std::fmt;

#[derive(Debug, Clone)]
pub struct Client {
    id: u16,
    available: f64, // hm, dangerous type, because of rounding errors, or not?
    held: f64,
    locked: bool,
}

impl Client {
    pub fn from_id(id: u16) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            locked: false,
        }
    }

    pub fn total(&self) -> f64 {
        self.available + self.held
    }

    pub fn available(&self) -> f64 {
        self.available
    }

    pub fn held(&self) -> f64 {
        self.held
    }

    pub fn as_tuple(&self) -> (u16, f64, f64, f64, bool) {
        (
            self.id,
            self.available,
            self.held,
            self.total(),
            self.locked,
        )
    }

    pub fn as_csv(&self) -> String {
        format!(
            "{},{:.4},{:.4},{:.4},{}",
            self.id,
            self.available,
            self.held,
            self.total(),
            self.locked,
        )
    }

    fn is_locked(&self) -> Result<(), &'static str> {
        if self.locked {
            Err("Client is locked")
        } else {
            Ok(())
        }
    }

    /// Adds `amount` to the clients available funds and returns the new available amount.
    pub fn deposit(&mut self, amount: f64) -> Result<f64, &'static str> {
        self.is_locked()?;
        self.available += amount;
        Ok(self.available)
    }

    /// Withdraws `amount` from the clients available funds and returns the new available amount.
    pub fn withdraw(&mut self, amount: f64) -> Result<f64, &'static str> {
        self.is_locked()?;
        if amount > self.available {
            Err("Amount is not available")
        } else {
            self.available -= amount;
            Ok(self.available)
        }
    }

    pub fn lock(&mut self) -> Result<(), String> {
        if self.locked {
            Err("Already Locked".to_string())
        } else {
            self.locked = true;
            Ok(())
        }
    }

    pub fn unlock(&mut self) -> Result<(), String> {
        if !self.locked {
            Err("Already Unlocked".to_string())
        } else {
            self.locked = false;
            Ok(())
        }
    }

    pub fn dispute(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.available {
            Err("Amount not available. Can't be held".to_string())
        } else {
            self.available -= amount;
            self.held += amount;
            Ok(())
        }
    }

    pub fn resolve(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.held {
            Err("Amount not available. Can't be resolved".to_string())
        } else {
            self.available += amount;
            self.held -= amount;
            Ok(())
        }
    }

    pub fn chargeback(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.held {
            Err("Amount not available. Can't be charged back".to_string())
        } else {
            self.held -= amount;
            self.lock()?;
            Ok(())
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Client {}: available: {:.4}, held: {:.4}, total: {:.4}, locked: {}",
            self.id,
            self.available,
            self.held,
            self.total(),
            self.locked
        )
    }
}
