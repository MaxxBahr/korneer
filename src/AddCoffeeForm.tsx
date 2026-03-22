import { useState } from "react";
import Container from "react-bootstrap/Container";
import Button from "react-bootstrap/Button";
import Form from "react-bootstrap/Form";
import React from "react";
import { invoke } from "@tauri-apps/api/core";

export default function AddCoffeeForm(){
    const [newcoffee, setNewcoffee] = useState({
        name: '',
        rating: 0,
        url: '',
        grind_size: 0,
        grind_time: 0.0,
        extraction_time: 0.0,
        // count: 0,
        // machine: '',
        taste: ''
    })

    function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) {
        const key = e.target.name;
        const value = e.target.value;
        let new_val;
        if (e.target.type === "number") {
            new_val = value === "" ? 0 : Number(value);
        }else{
            new_val = value;
        }
        const updatedCoffee = {...newcoffee, [key]: new_val};
        setNewcoffee(updatedCoffee)
        console.log(updatedCoffee);
    }
    console.log("Component rendered")

    function handleSubmit(e: React.FormEvent){
        e.preventDefault();
        console.log("New entry added!");
        invoke("add_new_entry", {coffee: newcoffee});
    }

    return(
        <Container>
            <Form onSubmit={handleSubmit}>
                <Form.Label htmlFor="name">name</Form.Label>
                <Form.Control type="text" name="name" id="name" onChange={handleChange}></Form.Control>

                <Form.Label htmlFor="rating">rating</Form.Label>
                <Form.Control type="number" name="rating" id="rating" onChange={handleChange}></Form.Control>

                <Form.Label htmlFor="taste">taste</Form.Label>
                <Form.Select aria-label="Default select example" name="taste" id="taste" onChange={handleChange}>
                                <option value="Default">Default</option>
                                <option value="Chocolate">Chocolate</option>
                                <option value="Fruity">Fruity</option>
                                <option value="Caramel">Caramel</option>
                                </Form.Select>

                {/* Change to number */}
                <Form.Label htmlFor="grind_time">grind time</Form.Label>
                <Form.Control type="number" name="grind_time" id="grind_time" onChange={handleChange}></Form.Control>

                {/* Change to number */}
                <Form.Label htmlFor="extraction_time">extraction time</Form.Label>
                <Form.Control type="number" name="extraction_time" id="extraction_time" onChange={handleChange}></Form.Control>

                {/* Change to increaser */}
                <Form.Label htmlFor="grind_size">grind size</Form.Label>
                <Form.Control type="number" name="grind_size" id="grind_size" onChange={handleChange}></Form.Control>

                <Form.Label htmlFor="url">URL</Form.Label>
                <Form.Control type="text" name="url" id="url" onChange={handleChange}></Form.Control>

                {/* Future Implementation */}
                {/* <Form.Label htmlFor="machine">machine</Form.Label>
                <Form.Control type="text" name="machine" id="machine" onChange={handleChange}></Form.Control> */}

                {/* Change to file upload */}
                <Form.Label htmlFor="photo">photo</Form.Label>
                <Form.Control type="text" name="photo" id="photo" onChange={handleChange}></Form.Control>
                <br />
                <Button variant="primary" type="submit">
                    Save changes
                </Button>
            </Form>
        </Container>
    )
}