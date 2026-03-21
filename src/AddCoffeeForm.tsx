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
        taste: '',
        grindtime: 0.0,
        extractiontime: 0.0,
        count: 0,
        url: '',
        machine: ''
    })

    function handleChange(e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) {
        const key = e.target.name;
        const value = e.target.value;
        const updatedCoffee = {...newcoffee, [key]: value};
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
                                <option>Default</option>
                                <option value="Chocolate">Chocolate</option>
                                <option value="Fruity">Fruity</option>
                                <option value="Caramel">Caramel</option>
                                </Form.Select>

                {/* Change to number */}
                <Form.Label htmlFor="grindtime">grind time</Form.Label>
                <Form.Control type="number" name="grindtime" id="grindtime" onChange={handleChange}></Form.Control>

                {/* Change to number */}
                <Form.Label htmlFor="extractiontime">extraction time</Form.Label>
                <Form.Control type="number" name="extractiontime" id="extractiontime" onChange={handleChange}></Form.Control>

                {/* Change to increaser */}
                <Form.Label htmlFor="count">count</Form.Label>
                <Form.Control type="number" name="count" id="count" onChange={handleChange}></Form.Control>

                <Form.Label htmlFor="url">URL</Form.Label>
                <Form.Control type="text" name="url" id="url" onChange={handleChange}></Form.Control>

                <Form.Label htmlFor="machine">machine</Form.Label>
                <Form.Control type="text" name="machine" id="machine" onChange={handleChange}></Form.Control>

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