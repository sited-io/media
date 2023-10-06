ALTER TABLE
  media_subscriptions
ADD
  COLUMN shop_id UUID NOT NULL,
ADD
  COLUMN stripe_subscription_id VARCHAR,
ADD
  COLUMN canceled_at TIMESTAMP WITH TIME ZONE;