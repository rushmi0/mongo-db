// Define the database name
const DATABASE = 'actixdb';
const COLLECTION = 'Event';

// Switch to the specified database
use(DATABASE);

// Create a new collection in the database
db.createCollection(COLLECTION);
// Supports options like schema validation, capped size, or TTL indexes


db.createUser({
    user: "dev_user",       // The username (should match .env: DB_USER)
    pwd: "dev_pass",        // The password (should match .env: DB_PASS)
    roles: [
        { role: "readWrite", db: "actixdb" } // Can be adjusted to other roles like read or dbAdmin
        // Create a new user for the database
        // The role assigned to the user:
        // - `readWrite`: Can read and write to the specified database
        // - `read`: Can only read data from the database
        // - `dbAdmin`: Can perform administrative tasks, such as creating indexes
        // - `userAdmin`: Can manage users and roles for the database
        // - Custom roles: You can define your own roles with specific permissions
    ]
});
