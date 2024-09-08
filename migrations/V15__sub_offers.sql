CREATE TABLE sub_offers(
  offer_id UUID NOT NULL PRIMARY KEY,
  shop_id UUID NOT NULL,
  user_id VARCHAR NOT NULL
);