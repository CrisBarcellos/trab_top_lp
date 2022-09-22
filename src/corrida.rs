#[derive(Debug)]
pub struct Carro {
    pub cor: String,
    km: u16,
    status: Status,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    Correndo,
    Parado,
}

pub trait Largada {
    fn correr(&mut self) -> Status;
}

impl Carro {
    pub fn novo(cor: String) -> Self {
        Carro {
            cor,
            km: 0,
            status: Status::Correndo,
        }
    }
}

impl Largada for Carro {
    fn correr(&mut self) -> Status {
        self.km = self.km + 1;
        let km = self.km;

        if km == 10 {
            self.status = Status::Parado;
        }

        self.status
    }
}
