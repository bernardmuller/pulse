use std::borrow::Cow;

/// Represents the domain for Garmin services
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GarminDomain {
    GarminCom,
    GarminCn,
    Custom(String),
}

impl Default for GarminDomain {
    fn default() -> Self {
        GarminDomain::GarminCom
    }
}

impl From<&str> for GarminDomain {
    fn from(s: &str) -> Self {
        match s {
            "garmin.com" => GarminDomain::GarminCom,
            "garmin.cn" => GarminDomain::GarminCn,
            custom => GarminDomain::Custom(custom.to_string()),
        }
    }
}

impl ToString for GarminDomain {
    fn to_string(&self) -> String {
        match self {
            GarminDomain::GarminCom => "garmin.com".to_string(),
            GarminDomain::GarminCn => "garmin.cn".to_string(),
            GarminDomain::Custom(domain) => domain.clone(),
        }
    }
}

/// Type alias for workout ID
pub type GCWorkoutId = String;

/// Enum representing the different base URL types in the Garmin Connect system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlBase {
    /// Connect Modern
    GcModern,
    /// Garmin SSO
    GarminSso,
    /// Garmin SSO Origin
    GarminSsoOrigin,
    /// Connect API
    GcApi,
}

/// URL builder that provides all Garmin Connect API endpoints
#[derive(Debug, Clone)]
pub struct UrlBuilder {
    domain: GarminDomain,
    gc_modern: String,
    garmin_sso_origin: String,
    gc_api: String,
}

impl UrlBuilder {
    /// Create a new UrlBuilder with the specified domain (defaults to garmin.com)
    pub fn new(domain: Option<GarminDomain>) -> Self {
        let domain = domain.unwrap_or_default();
        let domain_str = domain.to_string();

        let gc_modern = format!("https://connect.{}/modern", domain_str);
        let garmin_sso_origin = format!("https://sso.{}", domain_str);
        let gc_api = format!("https://connectapi.{}", domain_str);

        Self {
            domain,
            gc_modern,
            garmin_sso_origin,
            gc_api,
        }
    }

    /// Get the base URL for a specific URL type
    fn base_url(&self, base: UrlBase) -> Cow<'_, str> {
        match base {
            UrlBase::GcModern => Cow::Borrowed(&self.gc_modern),
            UrlBase::GarminSsoOrigin => Cow::Borrowed(&self.garmin_sso_origin),
            UrlBase::GarminSso => Cow::Owned(format!("{}/sso", self.garmin_sso_origin)),
            UrlBase::GcApi => Cow::Borrowed(&self.gc_api),
        }
    }

    /// Build a URL by combining a base URL with a path
    fn build_url(&self, base: UrlBase, path: &str) -> String {
        format!("{}{}", self.base_url(base), path)
    }

    // Basic URLs

    /// Get Garmin SSO URL
    pub fn garmin_sso(&self) -> String {
        self.base_url(UrlBase::GarminSso).into_owned()
    }

    /// Get Base URL for proxy operations
    pub fn base_url_proxy(&self) -> String {
        self.build_url(UrlBase::GcModern, "/proxy")
    }

    // API Endpoint URLs - All these are built using the same pattern

    /// Get Garmin SSO Embed URL
    pub fn garmin_sso_embed_url(&self) -> String {
        self.build_url(UrlBase::GarminSsoOrigin, "/sso/embed")
    }

    /// Get Signin URL
    pub fn signin_url(&self) -> String {
        self.build_url(UrlBase::GarminSso, "/signin")
    }

    /// Get Login URL
    pub fn login_url(&self) -> String {
        self.build_url(UrlBase::GarminSso, "/login")
    }

    /// Get OAuth URL
    pub fn oauth_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/oauth-service/oauth")
    }

    /// Get User Settings URL
    pub fn user_settings_url(&self) -> String {
        self.build_url(
            UrlBase::GcApi,
            "/userprofile-service/userprofile/user-settings/",
        )
    }

    /// Get User Profile URL
    pub fn user_profile_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/userprofile-service/socialProfile")
    }

    /// Get Activities URL
    pub fn activities_url(&self) -> String {
        self.build_url(
            UrlBase::GcApi,
            "/activitylist-service/activities/search/activities",
        )
    }

    /// Get Activity URL
    pub fn activity_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/activity-service/activity/")
    }

    /// Get Stat Activities URL
    pub fn stat_activities_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/fitnessstats-service/activity")
    }

    /// Get Download Zip URL
    pub fn download_zip_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/download-service/files/activity/")
    }

    /// Get Download GPX URL
    pub fn download_gpx_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/download-service/export/gpx/activity/")
    }

    /// Get Download TCX URL
    pub fn download_tcx_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/download-service/export/tcx/activity/")
    }

    /// Get Download KML URL
    pub fn download_kml_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/download-service/export/kml/activity/")
    }

    /// Get Upload URL
    pub fn upload_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/upload-service/upload/")
    }

    /// Get Import Data URL
    pub fn import_data_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/modern/import-data")
    }

    /// Get Daily Steps URL
    pub fn daily_steps_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/usersummary-service/stats/steps/daily/")
    }

    /// Get Daily Sleep URL
    pub fn daily_sleep_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/sleep-service/sleep/dailySleepData")
    }

    /// Get Daily Weight URL
    pub fn daily_weight_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/weight-service/weight/dayview")
    }

    /// Get Update Weight URL
    pub fn update_weight_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/weight-service/user-weight")
    }

    /// Get Daily Hydration URL
    pub fn daily_hydration_url(&self) -> String {
        self.build_url(
            UrlBase::GcApi,
            "/usersummary-service/usersummary/hydration/allData",
        )
    }

    /// Get Hydration Log URL
    pub fn hydration_log_url(&self) -> String {
        self.build_url(
            UrlBase::GcApi,
            "/usersummary-service/usersummary/hydration/log",
        )
    }

    /// Get Golf Scorecard Summary URL
    pub fn golf_scorecard_summary_url(&self) -> String {
        self.build_url(
            UrlBase::GcApi,
            "/gcs-golfcommunity/api/v2/scorecard/summary",
        )
    }

    /// Get Golf Scorecard Detail URL
    pub fn golf_scorecard_detail_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/gcs-golfcommunity/api/v2/scorecard/detail")
    }

    /// Get Daily Heart Rate URL
    pub fn daily_heart_rate_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/wellness-service/wellness/dailyHeartRate")
    }

    /// Get Workout URL, optionally with workout ID
    pub fn workout_url(&self, id: Option<&GCWorkoutId>) -> String {
        match id {
            Some(workout_id) => self.build_url(
                UrlBase::GcApi,
                &format!("/workout-service/workout/{}", workout_id),
            ),
            None => self.build_url(UrlBase::GcApi, "/workout-service/workout"),
        }
    }

    /// Get Workouts URL
    pub fn workouts_url(&self) -> String {
        self.build_url(UrlBase::GcApi, "/workout-service/workouts")
    }

    // Accessor methods for the private fields

    /// Get the domain
    pub fn domain(&self) -> &GarminDomain {
        &self.domain
    }

    /// Get the GC_MODERN URL
    pub fn gc_modern(&self) -> &str {
        &self.gc_modern
    }

    /// Get the GARMIN_SSO_ORIGIN URL
    pub fn garmin_sso_origin(&self) -> &str {
        &self.garmin_sso_origin
    }

    /// Get the GC_API URL
    pub fn gc_api(&self) -> &str {
        &self.gc_api
    }
}

impl Default for UrlBuilder {
    fn default() -> Self {
        Self::new(None)
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garmin_domain_default() {
        let domain = GarminDomain::default();
        assert_eq!(domain, GarminDomain::GarminCom);
    }

    #[test]
    fn test_garmin_domain_from_str() {
        assert_eq!(GarminDomain::from("garmin.com"), GarminDomain::GarminCom);
        assert_eq!(GarminDomain::from("garmin.cn"), GarminDomain::GarminCn);

        let custom = GarminDomain::from("example.com");
        if let GarminDomain::Custom(domain) = custom {
            assert_eq!(domain, "example.com");
        } else {
            panic!("Expected Custom variant");
        }
    }

    #[test]
    fn test_garmin_domain_to_string() {
        assert_eq!(GarminDomain::GarminCom.to_string(), "garmin.com");
        assert_eq!(GarminDomain::GarminCn.to_string(), "garmin.cn");
        assert_eq!(
            GarminDomain::Custom("test.com".to_string()).to_string(),
            "test.com"
        );
    }

    #[test]
    fn test_url_builder_default() {
        let builder = UrlBuilder::default();
        assert_eq!(*builder.domain(), GarminDomain::GarminCom);
        assert_eq!(builder.gc_modern(), "https://connect.garmin.com/modern");
        assert_eq!(builder.garmin_sso_origin(), "https://sso.garmin.com");
        assert_eq!(builder.gc_api(), "https://connectapi.garmin.com");
    }

    #[test]
    fn test_url_builder_new_with_domain() {
        let builder = UrlBuilder::new(Some(GarminDomain::GarminCn));
        assert_eq!(*builder.domain(), GarminDomain::GarminCn);
        assert_eq!(builder.gc_modern(), "https://connect.garmin.cn/modern");
        assert_eq!(builder.garmin_sso_origin(), "https://sso.garmin.cn");
        assert_eq!(builder.gc_api(), "https://connectapi.garmin.cn");
    }

    #[test]
    fn test_url_builder_new_with_custom_domain() {
        let builder = UrlBuilder::new(Some(GarminDomain::Custom("example.org".to_string())));
        if let GarminDomain::Custom(domain) = builder.domain() {
            assert_eq!(domain, "example.org");
        } else {
            panic!("Expected Custom domain");
        }
        assert_eq!(builder.gc_modern(), "https://connect.example.org/modern");
        assert_eq!(builder.garmin_sso_origin(), "https://sso.example.org");
        assert_eq!(builder.gc_api(), "https://connectapi.example.org");
    }

    #[test]
    fn test_base_urls() {
        let builder = UrlBuilder::default();
        assert_eq!(builder.garmin_sso(), "https://sso.garmin.com/sso");
        assert_eq!(
            builder.base_url_proxy(),
            "https://connect.garmin.com/modern/proxy"
        );
    }

    #[test]
    fn test_sso_endpoints() {
        let builder = UrlBuilder::default();

        assert_eq!(
            builder.garmin_sso_embed_url(),
            "https://sso.garmin.com/sso/embed"
        );
        assert_eq!(builder.signin_url(), "https://sso.garmin.com/sso/signin");
        assert_eq!(builder.login_url(), "https://sso.garmin.com/sso/login");
    }

    #[test]
    fn test_api_endpoints() {
        let builder = UrlBuilder::default();

        // Test a sample of API endpoints
        assert_eq!(
            builder.oauth_url(),
            "https://connectapi.garmin.com/oauth-service/oauth"
        );
        assert_eq!(
            builder.user_settings_url(),
            "https://connectapi.garmin.com/userprofile-service/userprofile/user-settings/"
        );
        assert_eq!(
            builder.activities_url(),
            "https://connectapi.garmin.com/activitylist-service/activities/search/activities"
        );
        assert_eq!(
            builder.daily_sleep_url(),
            "https://connectapi.garmin.com/sleep-service/sleep/dailySleepData"
        );
    }

    #[test]
    fn test_download_endpoints() {
        let builder = UrlBuilder::default();

        assert_eq!(
            builder.download_zip_url(),
            "https://connectapi.garmin.com/download-service/files/activity/"
        );
        assert_eq!(
            builder.download_gpx_url(),
            "https://connectapi.garmin.com/download-service/export/gpx/activity/"
        );
        assert_eq!(
            builder.download_tcx_url(),
            "https://connectapi.garmin.com/download-service/export/tcx/activity/"
        );
        assert_eq!(
            builder.download_kml_url(),
            "https://connectapi.garmin.com/download-service/export/kml/activity/"
        );
    }

    #[test]
    fn test_workout_endpoint() {
        let builder = UrlBuilder::default();

        // Test without ID
        assert_eq!(
            builder.workout_url(None),
            "https://connectapi.garmin.com/workout-service/workout"
        );

        // Test with ID
        let workout_id = "12345".to_string();
        assert_eq!(
            builder.workout_url(Some(&workout_id)),
            "https://connectapi.garmin.com/workout-service/workout/12345"
        );
    }

    #[test]
    fn test_workouts_endpoint() {
        let builder = UrlBuilder::default();
        assert_eq!(
            builder.workouts_url(),
            "https://connectapi.garmin.com/workout-service/workouts"
        );
    }

    #[test]
    fn test_with_different_domains() {
        // Test with garmin.cn domain
        let cn_builder = UrlBuilder::new(Some(GarminDomain::GarminCn));
        assert_eq!(
            cn_builder.activities_url(),
            "https://connectapi.garmin.cn/activitylist-service/activities/search/activities"
        );

        // Test with custom domain
        let custom_builder =
            UrlBuilder::new(Some(GarminDomain::Custom("test.garmin.io".to_string())));
        assert_eq!(
            custom_builder.activities_url(),
            "https://connectapi.test.garmin.io/activitylist-service/activities/search/activities"
        );
    }

    #[test]
    fn test_url_base_enum() {
        let builder = UrlBuilder::default();

        // Test using the internal base_url method directly
        assert_eq!(
            builder.base_url(UrlBase::GcModern),
            "https://connect.garmin.com/modern"
        );
        assert_eq!(
            builder.base_url(UrlBase::GarminSsoOrigin),
            "https://sso.garmin.com"
        );
        assert_eq!(
            builder.base_url(UrlBase::GarminSso),
            "https://sso.garmin.com/sso"
        );
        assert_eq!(
            builder.base_url(UrlBase::GcApi),
            "https://connectapi.garmin.com"
        );
    }
}
