// --------------
// CLUSTERS
// --------------

use alloc::vec::Vec;

use super::container::Container;

// good ref https://developers.redhat.com/blog/2020/05/11/top-10-must-know-kubernetes-design-patterns

struct Cluster {
    containers: Vec<Container>,
}

// randomisation heuristics

// given n users who want to access the database at once
// just choose a random one with a simple randomised hueristic
fn generic_database_access() {}
