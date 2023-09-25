-- Create the table with 10 columns of different data types
CREATE TABLE ExampleTable (
    ID SERIAL PRIMARY KEY,
    FirstName VARCHAR(50),
    LastName VARCHAR(50),
    Age INT,
    Salary DECIMAL(10, 2),
    BirthDate DATE,
    IsActive BOOLEAN,
    Email VARCHAR(100),
    PhoneNumber VARCHAR(20),
    Address TEXT
);

-- Insert 1000 rows into the table with random data
DO $$ 
DECLARE 
    counter INT := 1;
BEGIN
    WHILE counter <= 1000 LOOP
        INSERT INTO ExampleTable (FirstName, LastName, Age, Salary, BirthDate, IsActive, Email, PhoneNumber, Address)
        VALUES (
            'FirstName' || counter,
            'LastName' || counter,
            18 + (counter % 60), -- Age between 18 and 77
            50000.00 + (counter * 100), -- Salary starts at 50000.00 and increments
            CURRENT_DATE - (counter % 365), -- Random past date as BirthDate
            CASE WHEN counter % 2 = 0 THEN TRUE ELSE FALSE END, -- IsActive alternate between TRUE and FALSE
            'email' || counter || '@example.com',
            '123-555-' || LPAD(counter::TEXT, 4, '0'),
            'Address' || counter
        );
        counter := counter + 1;
    END LOOP;
END $$;