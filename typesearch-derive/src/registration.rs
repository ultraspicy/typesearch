#[macro_export]
  macro_rules! register_derived_fragment {
      (
          $fragment:ty,
          dependencies: [
              $(
                  $dep_fragment:ty => {
                      inject: |$builder:ident, $dep:ident| $inject_code:block,
                      triggers: [$($field:literal),*]
                  }
              ),*
          ]
      ) => {
          // Generate a registry entry
          inventory::submit! {
              DerivedFragmentRegistration {
                  fragment_name: stringify!($fragment),
                  dependencies: vec![
                      $(
                          DependencyRegistration {
                              source_type: stringify!($dep_fragment),
                              trigger_fields: vec![$($field.to_string()),*],
                              inject_fn: Box::new(|builder: &mut dyn Any, dep: &dyn Any| {
                                  let $builder = builder.downcast_mut::<
                                      <$fragment as DerivedFragment>::Builder
                                  >().unwrap();
                                  let $dep = dep.downcast_ref::<$dep_fragment>().unwrap();
                                  $inject_code
                              }),
                          }
                      ),*
                  ],
              }
          }
      };
  }

//   // Usage
//   register_derived_fragment! {
//       LoadStatusFragment,
//       dependencies: [
//           LoadFragment => {
//               inject: |builder, frag| {
//                   *builder = builder.clone().with_load_fragment(frag.clone());
//               },
//               triggers: ["status"]
//           },
//           JobAggregateFragment => {
//               inject: |builder, frag| {
//                   *builder = builder.clone().with_jobs(frag.clone());
//               },
//               triggers: ["job.statuses"]
//           },
//           StopsFragment => {
//               inject: |builder, frag| {
//                   *builder = builder.clone().with_stops(frag.clone());
//               },
//               triggers: ["pickup.status", "dropoff.status"]
//           }
//       ]
//   }
