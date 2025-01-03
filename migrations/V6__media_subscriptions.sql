CREATE TABLE media_subscriptions (
  media_subscription_id UUID PRIMARY KEY,
  buyer_user_id VARCHAR NOT NULL,
  offer_id UUID NOT NULL,
  current_period_start TIMESTAMP WITH TIME ZONE NOT NULL,
  current_period_end TIMESTAMP WITH TIME ZONE NOT NULL,
  subscription_status VARCHAR NOT NULL,
  payed_at TIMESTAMP WITH TIME ZONE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_shops_updated_at BEFORE UPDATE
    ON media_subscriptions FOR EACH ROW EXECUTE PROCEDURE 
    updated_at_now();
