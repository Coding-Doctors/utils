use std::default::Default;
use std::fmt;
use ::model::{ChannelId, Emoji, Mentionable, RoleId, UserId};

/// The Message Builder is an ergonomic utility to easily build a message,
/// by adding text and mentioning mentionable structs.
///
/// The finalized value can be accessed via [`build`] or the inner value.
///
/// # Examples
///
/// Build a message, mentioning a [`user`] and an [`emoji`], and retrieving the
/// value:
///
/// ```rust,ignore
/// use serenity::utils::MessageBuilder;
///
/// // assuming an `emoji` and `user` have already been bound
///
/// let content = MessageBuilder::new()
///     .push("You sent a message, ")
///     .mention(user)
///     .push("! ")
///     .mention(emoji)
///     .build();
/// ```
///
/// [`build`]: #method.build
/// [`emoji`]: #method.emoji
/// [`user`]: #method.user
pub struct MessageBuilder(pub String);

impl MessageBuilder {
    /// Creates a new, empty-content builder.
    pub fn new() -> MessageBuilder {
        MessageBuilder::default()
    }

    /// Pulls the inner value out of the builder.
    ///
    /// # Examples
    ///
    /// This is equivilant to simply retrieving the tuple struct's first value:
    ///
    /// ```rust
    /// use serenity::utils::MessageBuilder;
    ///
    /// let content = MessageBuilder::new().push("test").0;
    ///
    /// assert_eq!(content, "test");
    /// ```
    pub fn build(self) -> String {
        self.0
    }

    /// Mentions the [`GuildChannel`] in the built message.
    ///
    /// This accepts anything that converts _into_ a [`ChannelId`]. Refer to
    /// `ChannelId`'s documentation for more information.
    ///
    /// Refer to `ChannelId`'s [Display implementation] for more information on
    /// how this is formatted.
    ///
    /// [`ChannelId`]: ../model/struct.ChannelId.html
    /// [`GuildChannel`]: ../model/struct.GuildChannel.html
    /// [Display implementation]: ../model/struct.ChannelId.html#method.fmt-1
    pub fn channel<C: Into<ChannelId>>(mut self, channel: C) -> Self {
        self.0.push_str(&format!("{}", channel.into()));

        self
    }

    /// Displays the given emoji in the built message.
    ///
    /// Refer to `Emoji`s [Display implementation] for more information on how
    /// this is formatted.
    ///
    /// [Display implementation]: ../model/struct.Emoji.html#method.fmt
    pub fn emoji(mut self, emoji: Emoji) -> Self {
        self.0.push_str(&format!("{}", emoji));

        self
    }

    /// Mentions something that implements the [`Mentionable`] trait.
    ///
    /// [`Mentionable`]: ../model/trait.Mentionable.html
    pub fn mention<M: Mentionable>(mut self, item: M) -> Self {
        self.0.push_str(&item.mention());

        self
    }

    /// Pushes a string to the internal message content.
    ///
    /// Note that this does not mutate either the given data or the internal
    /// message content in anyway prior to appending the given content to the
    /// internal message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serenity::utils::MessageBuilder;
    ///
    /// let message = MessageBuilder::new().push("test");
    ///
    /// assert_eq!(message.push("ing").0, "testing");
    /// ```
    pub fn push(mut self, content: &str) -> Self {
        self.0.push_str(content);

        self
    }


    /// Mentions the [`Role`] in the built message.
    ///
    /// This accepts anything that converts _into_ a [`RoleId`]. Refer to
    /// `RoleId`'s documentation for more information.
    ///
    /// Refer to `RoleId`'s [Display implementation] for more information on how
    /// this is formatted.
    ///
    /// [`Role`]: ../model/struct.Role.html
    /// [`RoleId`]: ../model/struct.RoleId.html
    /// [Display implementation]: ../model/struct.RoleId.html#method.fmt-1
    pub fn role<R: Into<RoleId>>(mut self, role: R) -> Self {
        self.0.push_str(&format!("{}", role.into()));

        self
    }

    /// Mentions the [`User`] in the built message.
    ///
    /// This accepts anything that converts _into_ a [`UserId`]. Refer to
    /// `UserId`'s documentation for more information.
    ///
    /// Refer to `UserId`'s [Display implementation] for more information on how
    /// this is formatted.
    ///
    /// [`User`]: ../model/struct.User.html
    /// [`UserId`]: ../model/struct.UserId.html
    /// [Display implementation]: ../model/struct.UserId.html#method.fmt-1
    pub fn user<U: Into<UserId>>(mut self, user: U) -> Self {
        self.0.push_str(&format!("{}", user.into()));

        self
    }
}

impl fmt::Display for MessageBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Default for MessageBuilder {
    fn default() -> MessageBuilder {
        MessageBuilder(String::default())
    }
}
