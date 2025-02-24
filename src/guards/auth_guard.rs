use actix_session::SessionExt;
use actix_web::guard::{Guard, GuardContext};
use domain::value_objects::UserId;

pub struct AuthGuard;

impl Guard for AuthGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        if ctx
            .get_session()
            .get::<UserId>("user_id")
            .unwrap()
            .is_some()
        {
            ctx.get_session().renew();

            return true;
        }

        false
    }
}
