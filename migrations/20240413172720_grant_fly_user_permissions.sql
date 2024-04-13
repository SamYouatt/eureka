DO $$
DECLARE
    found_user pg_catalog.pg_roles%rowtype;
BEGIN
    SELECT * INTO found_user FROM pg_catalog.pg_roles WHERE rolname='sam_y_eureka';

    IF FOUND THEN
        GRANT ALL PRIVILEGES ON DATABASE eureka TO sam_y_eureka;
    END IF;
END $$;
