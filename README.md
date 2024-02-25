This is the `backend` part of the code

Here I will try `my best` to explain all the rust code you are seeing,like all the files and all

Ofc I had to keep the License because its fun

1. Database connections

     I am using a `Posgresql` database and I am using the diesel package to make changes to the DB,I am using pgadmin to view changes

     The simple SQL file is in the `migrations` folder where I have described a simple table

       `Up.sql - all the constructive db changes`
       `Down.sql - undo all the up changes`

     The database connection link is in a .env file,for safety reasons since I will be hosting this project on the cloud

     Rocket created a file called `src/schema.rs` that best describes the schema of the database

      
