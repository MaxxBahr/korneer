import 'bootstrap/dist/css/bootstrap.min.css';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Button from 'react-bootstrap/Button';
import Form from 'react-bootstrap/Form';

export default function Coffee() {
    return <Container>
        <Row className='justify-content-md-center mb-5'>
            <Col><h2>Photo</h2></Col>
            <Col>
                <Col><h2>House Blend</h2></Col>
                <Row>
                    <Col>Rating:</Col>
                    <Col>taste:     <Form.Select aria-label="Default select example">
                            <option>Default</option>
                            <option value="Chocolate">Chocolate</option>
                            <option value="Fruity">Fruity</option>
                            <option value="Caramel">Caramel</option>
                            </Form.Select>
                    </Col>
                </Row>
                <Row>
                    <Col>grind time:</Col>
                    <Col>extraction time:</Col>
                </Row>
                <Row>
                    <Col>count:</Col>
                    <Col><Button>+</Button></Col>
                </Row>
                <Row>
                    <Col>url:</Col>
                </Row>
                <Row>
                    <Col>machine:</Col>
                </Row>
            </Col>
        </Row>
        
    </Container>;
}