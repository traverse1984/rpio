#[macro_export]
macro_rules! pinout {
   () => {};

   (
       input pulldown { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
      $(let $name = $pin.into_pull_down_input();)*
      $crate::pinout!($($tail)*)
   };

   (
       input pullup { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(let $name = $pin.into_pull_up_input();)*
       $crate::pinout!($($tail)*)
   };

   (
       input floating { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(let $name = $pin.into_floating_input();)*
       $crate::pinout!($($tail)*)
   };

   (
       output { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
      $(let mut $name = $pin.into_push_pull_output();)*
      $crate::pinout!($($tail)*)
   };

   (
       output low { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(
           let mut $name = {
               let mut pin = $pin.into_push_pull_output();
               pin.set_low().ok().unwrap();
               pin
           };
       )*
       $crate::pinout!($($tail)*)
   };

   (
       output high { $($name: ident = $pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
       $(
           let mut $name = {
               let mut pin = $pin.into_push_pull_output();
               pin.set_high().ok().unwrap();
               pin
           };
       )*
       $crate::pinout!($($tail)*)
   };

   (
      disabled floating { $($pin: expr),* $(,)? }
      $($tail: tt)*
   ) => {
      $($pin.into_floating_disabled();)*
      $crate::pinout!($($tail)*)
   };

   (
      disabled pulldown { $($pin: expr),* $(,)? }
      $($tail: tt)*
   ) => {
      $($pin.into_pull_down_disabled();)*
      $crate::pinout!($($tail)*)
   };

   (
      disabled pullup { $($pin: expr),* $(,)? }
      $($tail: tt)*
   ) => {
      $($pin.into_pull_up_disabled();)*
      $crate::pinout!($($tail)*)
   };

   (
       spi { $($pin: expr),* $(,)? }
       $($tail: tt)*
   ) => {
      $(
          let _ = $pin.into_mode::<rp2040_hal::gpio::FunctionSpi>();
       )*
       $crate::pinout!($($tail)*)
   };

}
