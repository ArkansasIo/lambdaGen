pub fn distance(a:(f64,f64),b:(f64,f64))->f64{((a.0-b.0).powi(2)+(a.1-b.1).powi(2)).sqrt()}
pub fn triangle_area(a:(f64,f64),b:(f64,f64),c:(f64,f64))->f64{((a.0*(b.1-c.1)+b.0*(c.1-a.1)+c.0*(a.1-b.1))/2.0).abs()}
pub fn orientation(a:(f64,f64),b:(f64,f64),c:(f64,f64))->f64{(b.0-a.0)*(c.1-a.1)-(b.1-a.1)*(c.0-a.0)}
