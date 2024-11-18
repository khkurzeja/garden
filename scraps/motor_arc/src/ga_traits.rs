#![allow(dead_code)]

pub trait Neg<A> { 
    type R;
    fn neg(a: A) -> Self::R;
}
pub fn neg<A>(a: A) -> <() as Neg::<A>>::R where (): Neg<A> {
    <() as Neg::<A>>::neg(a)
}

pub trait Inverse<A> { 
    type R;
    fn inverse(a: A) -> Self::R;
}
pub fn inverse<A>(a: A) -> <() as Inverse::<A>>::R where (): Inverse<A> {
    <() as Inverse::<A>>::inverse(a)
}

pub trait SqMagnitude<A> { 
    type R;
    fn sq_magnitude(a: A) -> Self::R;
}
pub fn sq_magnitude<A>(a: A) -> <() as SqMagnitude::<A>>::R where (): SqMagnitude<A> {
    <() as SqMagnitude::<A>>::sq_magnitude(a)
}

pub trait Magnitude<A> { 
    type R;
    fn magnitude(a: A) -> Self::R;
}
pub fn magnitude<A>(a: A) -> <() as Magnitude::<A>>::R where (): Magnitude<A> {
    <() as Magnitude::<A>>::magnitude(a)
}

pub trait Add<A, B> { 
    type R;
    fn add(a: A, b: B) -> Self::R;
}
pub fn add<A, B>(a: A, b: B) -> <() as Add::<A, B>>::R where (): Add<A, B> {
    <() as Add::<A, B>>::add(a, b)
}

pub trait Sub<A, B> { 
    type R;
    fn sub(a: A, b: B) -> Self::R;
}
pub fn sub<A, B>(a: A, b: B) -> <() as Sub::<A, B>>::R where (): Sub<A, B> {
    <() as Sub::<A, B>>::sub(a, b)
}

pub trait Mul<A, B> { 
    type R;
    fn mul(a: A, b: B) -> Self::R;
}
pub fn mul<A, B>(a: A, b: B) -> <() as Mul::<A, B>>::R where (): Mul<A, B> {
    <() as Mul::<A, B>>::mul(a, b)
}

pub trait Antimul<A, B> { 
    type R;
    fn antimul(a: A, b: B) -> Self::R;
}
pub fn antimul<A, B>(a: A, b: B) -> <() as Antimul::<A, B>>::R where (): Antimul<A, B> {
    <() as Antimul::<A, B>>::antimul(a, b)
}

pub trait Div<A, B> { 
    type R;
    fn div(a: A, b: B) -> Self::R;
}
pub fn div<A, B>(a: A, b: B) -> <() as Div::<A, B>>::R where (): Div<A, B> {
    <() as Div::<A, B>>::div(a, b)
}

pub trait Wedge<A, B> { 
    type R;
    fn wedge(a: A, b: B) -> Self::R;
}
pub fn wedge<A, B>(a: A, b: B) -> <() as Wedge::<A, B>>::R where (): Wedge<A, B> {
    <() as Wedge::<A, B>>::wedge(a, b)
}

pub trait Antiwedge<A, B> { 
    type R;
    fn antiwedge(a: A, b: B) -> Self::R;
}
pub fn antiwedge<A, B>(a: A, b: B) -> <() as Antiwedge::<A, B>>::R where (): Antiwedge<A, B> {
    <() as Antiwedge::<A, B>>::antiwedge(a, b)
}

pub trait RightComplement<A> { 
    type R;
    fn right_complement(a: A) -> Self::R;
}
pub fn right_complement<A>(a: A) -> <() as RightComplement::<A>>::R where (): RightComplement<A> {
    <() as RightComplement::<A>>::right_complement(a)
}

pub trait LeftComplement<A> { 
    type R;
    fn left_complement(a: A) -> Self::R;
}
pub fn left_complement<A>(a: A) -> <() as LeftComplement::<A>>::R where (): LeftComplement<A> {
    <() as LeftComplement::<A>>::left_complement(a)
}

pub trait Reverse<A> { 
    type R;
    fn reverse(a: A) -> Self::R;
}
pub fn reverse<A>(a: A) -> <() as Reverse::<A>>::R where (): Reverse<A> {
    <() as Reverse::<A>>::reverse(a)
}

pub trait Dot<A, B> { 
    type R;
    fn dot(a: A, b: B) -> Self::R;
}
pub fn dot<A, B>(a: A, b: B) -> <() as Dot::<A, B>>::R where (): Dot<A, B> {
    <() as Dot::<A, B>>::dot(a, b)
}

pub trait Antidot<A, B> { 
    type R;
    fn antidot(a: A, b: B) -> Self::R;
}
pub fn antidot<A, B>(a: A, b: B) -> <() as Antidot::<A, B>>::R where (): Antidot<A, B> {
    <() as Antidot::<A, B>>::antidot(a, b)
}


// Projective GA stuff

pub trait Bulk<A> { 
    type R;
    fn bulk(a: A) -> Self::R;
}
pub fn bulk<A>(a: A) -> <() as Bulk::<A>>::R where (): Bulk<A> {
    <() as Bulk::<A>>::bulk(a)
}

pub trait Weight<A> { 
    type R;
    fn weight(a: A) -> Self::R;
}
pub fn weight<A>(a: A) -> <() as Weight::<A>>::R where (): Weight<A> {
    <() as Weight::<A>>::weight(a)
}

pub trait BulkDual<A> { 
    type R;
    fn bulk_dual(a: A) -> Self::R;
}
pub fn bulk_dual<A>(a: A) -> <() as BulkDual::<A>>::R where (): BulkDual<A> {
    <() as BulkDual::<A>>::bulk_dual(a)
}

pub trait WeightDual<A> { 
    type R;
    fn weight_dual(a: A) -> Self::R;
}
pub fn weight_dual<A>(a: A) -> <() as WeightDual::<A>>::R where (): WeightDual<A> {
    <() as WeightDual::<A>>::weight_dual(a)
}

pub trait BulkContract<A, B> { 
    type R;
    fn bulk_contract(a: A, b: B) -> Self::R;
}
pub fn bulk_contract<A, B>(a: A, b: B) -> <() as BulkContract::<A, B>>::R where (): BulkContract<A, B> {
    <() as BulkContract::<A, B>>::bulk_contract(a, b)
}

pub trait WeightContract<A, B> { 
    type R;
    fn weight_contract(a: A, b: B) -> Self::R;
}
pub fn weight_contract<A, B>(a: A, b: B) -> <() as WeightContract::<A, B>>::R where (): WeightContract<A, B> {
    <() as WeightContract::<A, B>>::weight_contract(a, b)
}

pub trait BulkExpand<A, B> { 
    type R;
    fn bulk_expand(a: A, b: B) -> Self::R;
}
pub fn bulk_expand<A, B>(a: A, b: B) -> <() as BulkExpand::<A, B>>::R where (): BulkExpand<A, B> {
    <() as BulkExpand::<A, B>>::bulk_expand(a, b)
}

pub trait WeightExpand<A, B> { 
    type R;
    fn weight_expand(a: A, b: B) -> Self::R;
}
pub fn weight_expand<A, B>(a: A, b: B) -> <() as WeightExpand::<A, B>>::R where (): WeightExpand<A, B> {
    <() as WeightExpand::<A, B>>::weight_expand(a, b)
}

pub trait Project<A, B> { 
    type R;
    fn project(a: A, b: B) -> Self::R;
}
pub fn project<A, B>(a: A, b: B) -> <() as Project::<A, B>>::R where (): Project<A, B> {
    <() as Project::<A, B>>::project(a, b)
}

pub trait Antiproject<A, B> { 
    type R;
    fn antiproject(a: A, b: B) -> Self::R;
}
pub fn antiproject<A, B>(a: A, b: B) -> <() as Antiproject::<A, B>>::R where (): Antiproject<A, B> {
    <() as Antiproject::<A, B>>::antiproject(a, b)
}

pub trait Unitize<A> { 
    type R;
    fn unitize(a: A) -> Self::R;
}
pub fn unitize<A>(a: A) -> <() as Unitize::<A>>::R where (): Unitize<A> {
    <() as Unitize::<A>>::unitize(a)
}

pub trait Attitude<A> { 
    type R;
    fn attitude(a: A) -> Self::R;
}
pub fn attitude<A>(a: A) -> <() as Attitude::<A>>::R where (): Attitude<A> {
    <() as Attitude::<A>>::attitude(a)
}

pub trait BulkNormSq<A> { 
    type R;
    fn bulk_norm_sq(a: A) -> Self::R;
}
pub fn bulk_norm_sq<A>(a: A) -> <() as BulkNormSq::<A>>::R where (): BulkNormSq<A> {
    <() as BulkNormSq::<A>>::bulk_norm_sq(a)
}

pub trait WeightNormSq<A> { 
    type R;
    fn weight_norm_sq(a: A) -> Self::R;
}
pub fn weight_norm_sq<A>(a: A) -> <() as WeightNormSq::<A>>::R where (): WeightNormSq<A> {
    <() as WeightNormSq::<A>>::weight_norm_sq(a)
}

pub trait NormSq<A> { 
    type R;
    fn norm_sq(a: A) -> Self::R;
}
pub fn norm_sq<A>(a: A) -> <() as NormSq::<A>>::R where (): NormSq<A> {
    <() as NormSq::<A>>::norm_sq(a)
}

pub trait Distance<A, B> { 
    type R;
    fn distance(a: A, b: B) -> Self::R;
}
pub fn distance<A, B>(a: A, b: B) -> <() as Distance::<A, B>>::R where (): Distance<A, B> {
    <() as Distance::<A, B>>::distance(a, b)
}

pub trait DistanceSq<A, B> { 
    type R;
    fn distance_sq(a: A, b: B) -> Self::R;
}
pub fn distance_sq<A, B>(a: A, b: B) -> <() as DistanceSq::<A, B>>::R where (): DistanceSq<A, B> {
    <() as DistanceSq::<A, B>>::distance_sq(a, b)
}

pub trait CosOfAngleSq<A, B> { 
    type R;
    fn cos_of_angle_sq(a: A, b: B) -> Self::R;
}
pub fn cos_of_angle_sq<A, B>(a: A, b: B) -> <() as CosOfAngleSq::<A, B>>::R where (): CosOfAngleSq<A, B> {
    <() as CosOfAngleSq::<A, B>>::cos_of_angle_sq(a, b)
}