# Current Understanding

## **proto** 


### **proto.rs**

Define what data will be stored inside the blockchain. Messages described in this file will be serialized before entering the blockchain/being transferred between nodes/being transferred from a light client. Messages are serialized into a binary format.

### **mod.rs** 

Defines where protobuf generated rust files (which are used for data serialization and deserialization) are outputed (output to **protobuf_mod.rs**).



## **root**



### **build.rs**

Where we introduce a main function to generate rust files from proto descriptions. We define an output of **protobuf_mod.rs** and a source of **src/proto**.

The dependency **ProtobufGenerator** generates the corresponding rs files from our proto definitions.  


## **src**



### **lib.rs**

Main entry point of the application.

### **wallets.rs**

Where we provide the validation for wallet proto messages. The **ProtobufConvert** dependency allows us to map our structures and structures generated from our **service.proto**. Validation is provided by making sure defined structures here match those defined in the proto file, where structs here must have the same fields as those in proto.

We also define an implementation of wallet methods, where one is used to increase balance, and one is used to decrease. Consider this section almost like a constructor in Java, with two associated methods.

### **schema.rs**

Where we provide a structured view of data storage. We dont access the storage directly, but instead we use **Access**. **Access** wraps udnerlying data access types like **Snapshot** (which provides an immutable view of data in our db) and **Fork** (which provides a mutable view of data in our db).

Our structured view of wallets in **schema.rs** is identical to the layout of the db in storage, so we dont need to write code to connect it to our db. Instead, we use the **FromAccess** trait from the **exonum_derive** dependency to generate this code for us.

Data will be stored in a key value format the MerkleDB index **MapIndex**. This data is stored as serialized wallet structures. 

**FromAccess** provides a method known as **from_root**, which can allow us to initialize our **CurrencySchema**. We also create a constructor using this method, to simplify interaction with **CurrencySchema**.

https://exonum.com/doc/version/latest/architecture/merkledb/#mapindex

### **transactions.rs**

Where we define our transactions, which alter our blockchain state. First, we must update our **service.proto** file with the new transaction messages. 

We create two transactions. The **TxCreateWallet** struct validates the create wallet message, whilst the **TxTransfer** struct validates the transferring money between wallets message. The **TxTransfer** struct contains the public key of the reciever. 

Transactions must be **Authorized** and **Authenticated**. In this case, **Authorization** occurs when the owner of a cryptocurrency signs the transfer transaction with a key associated with coins. **Authentication** means verifying that this transaction is signed with a specific key.

https://exonum.com/doc/version/latest/architecture/transactions/

### **service_interface.rs**

Here we declare a service interface (called **CryptocurrencyInterface**) to support the previously described transactions. Like smart contracts in other blockchain platforms, services provide the business logic of the application. It's essentially what defines transaction functionality. 

The **CryptocurrencyInterface** is what allows us to communicate with the external world, with it's implementation allowing us to transfer money between wallets, or to create new walllets. Calls made to service methods must produce an identical result on all nodes in the network given the same blockchain state. 

**exonum_interface** allows us to dispatch transactions and deserialize their payloads within the service (most probably due to transactions being recieved as protobuf messages). **interface_method** will specify an ID for each transaction method. Each ID should be unique. Each transaction method should have a signiture of the following format:

```
fn create_wallet(&self, ctx: Ctx, arg: TxCreateWallet) -> Self::Output;
```
We implement our service interface by actually declaring a service (which is a struct that we call **CryptocurrencyService**), and then use this to implement the **CryptocurrencyInterface** trait that represents the service interface. **service_dispatcher** collects info about interfaces implemented by our service, and **service_factory** generates code to create instances of our service.


We also define a struct called **Service**, which implements traits defined by rust runtime. Implementation of this trait contains additional information on the service lifecycle, like wiring our API.

We now begin implementing the methods defined in our service interface (**CryptocurrencyInterface**) using our **CryptocurrencyService**. We define our first transactions logic, which is creating a wallet. Here we check if a wallet exists, and add a new wallet if it doesnt. 

This transaction also sets the wallet balance to 100. To work with our db, we instatsiate **CurrencySchema** (defined in schema.rs) using the **service_data** method of **ExecutionContext**.

We then begin defining how to transfer money between wallets. We first check to see if wallets on both sides exist, check the balance of the sender to see if they have enough tokens, and then decrease their token amount while increasing that of the reciever. There also needs to be a check to make sure that a sender doesnt send money to themselves.

https://exonum.com/doc/version/latest/architecture/services/

https://docs.rs/exonum-rust-runtime/latest/exonum_rust_runtime/

### **errors.rs**

Where we define erros regarding blockchain transaction failure. 

### **cryptoapi.rs**

We now implement the node API. We declare a blank struct called **CryptoCurrencyApi**, which will include methods that correspond to different types of requests.

We want to implement 2 read requests for **CryptocurrencyService**. This requests must return info of all wallets in the system, and of a specific wallet using a public key. We do this by defining these methods in our **CryptocurencyApi**, which use **state** to read info from blockchain storage. The **state** contains an interface known as **ServiceApiState** to access blockchain data.

We also define a helper struct, called **WalletQuery**. This structure describes the query parameters for the **get_wallet** endpoint, (**pub_key**). 

These read methods have an idiomatic signiture, just like the transaction endpoints.

Read request endpoint signiture:

```
fn(&ServiceApiState<'_>, Query) -> api::Result<Response>;
```

Transaction endpoints signiture: 

```
fn create_wallet(&self, ctx: Ctx, arg: TxCreateWallet) -> Self::Output;
```
This allows for easy to udnerstand and repoduce code between different developers.

Inside our **CryptocurrencyApi** struct impl, we add in a helper (**called wire**) to help wire the API with endpoints. We then tie the request processing logic to the specific endpoints by adding a **wire_api** method to our **Service** impl in **service_interface.rs**. This will call the **wire** method that we just added to our **CryptocurrencyAPi** struct.













