
trait DerivedFragment {
    type Index: IndexEntity;
    type Builder: FragmentBuilder<Output = Self>;

    // List dependencies at compile time
    const DEPENDENCIES: &'static [DependencySpec];
}

struct DependencySpec {
    fragment_type: &'static str,
    trigger_fields: &'static [&'static str],
}

//
//  struct LoadStatusFragment {
//       uuid: String,
//       load_status: String,
//   }

//   impl DerivedFragment for LoadStatusFragment {
//       type Index = Load;
//       type Builder = LoadStatusBuilder;

//       const DEPENDENCIES: &'static [DependencySpec] = &[
//           DependencySpec {
//               fragment_type: "LoadFragment",
//               trigger_fields: &["status"],
//           },
//           DependencySpec {
//               fragment_type: "JobAggregateFragment",
//               trigger_fields: &["job.statuses"],
//           },
//           DependencySpec {
//               fragment_type: "StopsFragment",
//               trigger_fields: &["pickup.status", "dropoff.status"],
//           },
//       ];
//   }

//  struct LoadStatusBuilder {
//       uuid: String,
//       load_fragment: Option<LoadFragment>,
//       jobs: Option<JobAggregateFragment>,
//       stops: Option<StopsFragment>,
//   }

//   impl LoadStatusBuilder {
//       fn with_load_fragment(mut self, fragment: LoadFragment) -> Self {
//           self.load_fragment = Some(fragment);
//           self
//       }

//       fn with_jobs(mut self, jobs: JobAggregateFragment) -> Self {
//           self.jobs = Some(jobs);
//           self
//       }

//       fn with_stops(mut self, stops: StopsFragment) -> Self {
//           self.stops = Some(stops);
//           self
//       }

//       fn build(self) -> LoadStatusFragment {
//           LoadStatusFragment {
//               uuid: self.uuid.clone(),
//               load_status: derive_load_status(
//                   self.load_fragment.as_ref(),
//                   self.jobs.as_ref(),
//                   self.stops.as_ref(),
//               ),
//           }
//       }
//   }

//todo make it a derive attr like instead of manual wiring 

//   #[derived_fragment(index = Load)]
//   struct LoadStatusFragment {
//       #[dependency(fragment = LoadFragment, triggers = ["status"])]
//       load: LoadFragment,

//       #[dependency(fragment = JobAggregateFragment, triggers = ["job.statuses"])]
//       jobs: JobAggregateFragment,

//       #[dependency(fragment = StopsFragment, triggers = ["pickup.status", "dropoff.status"])]
//       stops: StopsFragment,

//       // Computed field
//       #[computed]
//       load_status: String,
//   }
