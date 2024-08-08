/// Different events to which a user can subscribe to.
pub enum NotificationEvents {
    /// Notifications for interlude phase.
    InterludePhase(PhaseNotification),
    /// Notifications for leadin phase.
    LeadinPhaseStart(PhaseNotification),
    /// Notifications for fixed price phase.
    FixedPhaseStart(PhaseNotification),
    /// Whenever coretime is sold.
    CoretimeSale,
}

pub enum PhaseNotification {
    /// Getting a notification `u64` seconds prior to phase start.
    PriorStart(u64)
    /// Getting a notification `u64` seconds prior to phase end.
    PriorEnd(u64)
}

/// Available options for receiving notification prior to an event happening.
pub enum PriorTimeOptions {
    /// Receive a notification day before the phase starts.
    DayAhead,
    /// Receive a notification 12 hours before the phase starts.
    HalfDayAhead,
    /// Receive a notification 6 hours before the phase starts.
    QuarterDayAhead,
    /// Receive a notification one hour before the phase starts.
    HourAhead,
}
