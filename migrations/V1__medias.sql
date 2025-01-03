-- updated_at_now() was created in commerce

CREATE TABLE medias (
  media_id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  market_booth_id UUID NOT NULL,
  user_id VARCHAR NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  name VARCHAR NOT NULL,
  data_url VARCHAR NOT NULL
);

CREATE TRIGGER update_shops_updated_at BEFORE UPDATE
    ON medias FOR EACH ROW EXECUTE PROCEDURE 
    updated_at_now();
