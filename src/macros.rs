#[doc(hidden)]
#[macro_export]
macro_rules! successor (
  (0 $id:ident $($rest:tt)+) => { $id!(1 $($rest)+); };
  (1 $id:ident $($rest:tt)+) => { $id!(2 $($rest)+); };
  (2 $id:ident $($rest:tt)+) => { $id!(3 $($rest)+); };
  (3 $id:ident $($rest:tt)+) => { $id!(4 $($rest)+); };
  (4 $id:ident $($rest:tt)+) => { $id!(5 $($rest)+); };
  (5 $id:ident $($rest:tt)+) => { $id!(6 $($rest)+); };
  (6 $id:ident $($rest:tt)+) => { $id!(7 $($rest)+); };
  (7 $id:ident $($rest:tt)+) => { $id!(8 $($rest)+); };
  (8 $id:ident $($rest:tt)+) => { $id!(9 $($rest)+); };
  (9 $id:ident $($rest:tt)+) => { $id!(10 $($rest)+); };
  (10 $id:ident $($rest:tt)+) => { $id!(11 $($rest)+); };
  (11 $id:ident $($rest:tt)+) => { $id!(12 $($rest)+); };
  (12 $id:ident $($rest:tt)+) => { $id!(13 $($rest)+); };
  (13 $id:ident $($rest:tt)+) => { $id!(14 $($rest)+); };
  (14 $id:ident $($rest:tt)+) => { $id!(15 $($rest)+); };
  (15 $id:ident $($rest:tt)+) => { $id!(16 $($rest)+); };
  (16 $id:ident $($rest:tt)+) => { $id!(17 $($rest)+); };
  (17 $id:ident $($rest:tt)+) => { $id!(18 $($rest)+); };
  (18 $id:ident $($rest:tt)+) => { $id!(19 $($rest)+); };
  (19 $id:ident $($rest:tt)+) => { $id!(20 $($rest)+); };
  (20 $id:ident $($rest:tt)+) => { $id!(21 $($rest)+); };
  (21 $id:ident $($rest:tt)+) => {  };
);

#[doc(hidden)]
#[macro_export]
macro_rules! execute_for_tuples (
  ($id:ident) => {
    $id! {
      T1: R1,
      T2: R2,
      T3: R3,
      T4: R4,
      T5: R5,
      T6: R6,
      T7: R7,
      T8: R8,
      T9: R9,
      T10: R10,
      T11: R11,
      T12: R12,
      T13: R13,
      T14: R14,
      T15: R15,
      T16: R16,
      T17: R17,
      T18: R18,
      T19: R19,
      T20: R20
    }
  };
);
