// patterns can be refutable or irrefutable

// an irrefutable pattern cannot feel, e.g. let x = 5;
// a refutable pattern could fail, e.g. if let Some(x) = y
// in this case, we need to create a scope that runs if the
// pattern succeeds

// match arms must use refutable patterns except for last arm

// welp, no code