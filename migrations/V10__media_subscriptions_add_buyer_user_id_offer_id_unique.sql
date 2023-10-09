ALTER TABLE
  media_subscriptions
ADD
  UNIQUE (buyer_user_id, offer_id);