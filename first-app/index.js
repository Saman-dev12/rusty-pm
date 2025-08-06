const express = require("express")

const app = express()

app.get("/",(req,res)=>{
	res.send("Hello world");
})

app.listen(8089,()=>{
	console.log("Server is listening at http://localhost:8089");
})
console.log("hello")
