#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // Para lidar com panics em sistemas embarcados
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio::{gpiob::PB0, Output, PushPull, PB1, PB5, PB6},
    pac,
    prelude::*,
    serial::{Config, Serial},
};

struct Robot {
    relay1: PB0<Output<PushPull>>,
    relay2: PB1<Output<PushPull>>,
    relay3: PB5<Output<PushPull>>,
    relay4: PB6<Output<PushPull>>,
}

impl Robot {
    fn new(
        relay1: PB0<Output<PushPull>>,
        relay2: PB1<Output<PushPull>>,
        relay3: PB5<Output<PushPull>>,
        relay4: PB6<Output<PushPull>>,
    ) -> Self {
        Self {
            relay1,
            relay2,
            relay3,
            relay4,
        }
    }

    fn move_forward(&mut self) {
        rprintln!("Moving forward");
        self.relay1.set_high();
        self.relay2.set_low();
        self.relay3.set_high();
        self.relay4.set_low();
    }

    fn move_backward(&mut self) {
        rprintln!("Moving backward");
        self.relay1.set_low();
        self.relay2.set_high();
        self.relay3.set_low();
        self.relay4.set_high();
    }

    fn turn_left(&mut self) {
        rprintln!("Turning left");
        self.relay1.set_low();
        self.relay2.set_high();
        self.relay3.set_high();
        self.relay4.set_low();
    }

    fn turn_right(&mut self) {
        rprintln!("Turning right");
        self.relay1.set_high();
        self.relay2.set_low();
        self.relay3.set_low();
        self.relay4.set_high();
    }

    fn stop(&mut self) {
        rprintln!("Stopping");
        self.relay1.set_low();
        self.relay2.set_low();
        self.relay3.set_low();
        self.relay4.set_low();
    }
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();
    let mut afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    // Configura o clock do sistema
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx_pin = gpioa.pa10;

    // Configura os pinos como saídas para controlar os relés
    let relay1 = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let relay2 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    let relay3 = gpiob.pb5.into_push_pull_output(&mut gpiob.crl);
    let relay4 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);

    let mut robot = Robot::new(relay1, relay2, relay3, relay4);

    rtt_init_print!();

    let serial_config = Config::default().baudrate(9600.bps());
    let mut serial = Serial::new(
        dp.USART1,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        serial_config,
        &clocks,
    );

    rprintln!("Sistema iniciado!");
    loop {
        // Verifica se há dados disponíveis
        if let Ok(data) = serial.read() {
            rprintln!("Recebido: {}", data);

            match data {
                b'F' => robot.move_forward(),
                b'B' => robot.move_backward(),
                b'L' => robot.turn_left(),
                b'R' => robot.turn_right(),
                b'S' => robot.stop(),
                _ => rprintln!("Comando não reconhecido: {}", data as char),
            }
        } else {
            rprintln!("Aguardando dados...");
            // Pode incluir um pequeno atraso aqui, se necessário
        }
    }
}
