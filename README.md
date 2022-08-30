# Current Understanding

## **proto** 

</br>

## proto.rs 


Define what data will be stored inside the blockchain. Messages described in this file will be serialized before entering the blockchain/being transferred between nodes/being transferred from a light client. Messages are serialized into a binary format.

## mod.rs 

Defines where protobuf generated rust files (which are used for data serialization and deserialization) are outputed (output to **protobuf_mod.rs**).

</br>

## **root**

</br>

## build.rs

Where we introduce a main function to generate rust files from proto descriptions. We define an output of **protobuf_mod.rs** and a source of **src/proto**.

The dependency **ProtobufGenerator** generates the corresponding rs files from our proto definitions.  

</br>

## **src**

</br>

## lib.rs

Main entry point of the application.

## wallets.rs

Where we provide the validation for wallet proto messages. The **ProtobufConvert** dependency allows us to map our structures and structures generated from our **service.proto**. Validation is provided by making sure defined structures here match those defined in the proto file, where structs here must have the same fields as those in proto.

We also define an implementation of wallet methods, where one is used to increase balance, and one is used to decrease. Consider this section almost like a constructor in Java, with two associated methods.











