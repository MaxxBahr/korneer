import Button from 'react-bootstrap/Button';
import Modal from 'react-bootstrap/Modal';
import AddCoffeeForm from './AddCoffeeForm';

function AddCoffee({ show, handleClose }: {show: boolean, handleClose: ()=> void}) {
  return (
    <Modal show={show} onHide={handleClose}>
      <Modal.Header closeButton>
        <Modal.Title>Add new coffee</Modal.Title>
      </Modal.Header>

      <Modal.Body>
        <AddCoffeeForm></AddCoffeeForm>
      </Modal.Body>

      <Modal.Footer>
        <Button variant="secondary" onClick={handleClose}>
          Abort
        </Button>
        <Button variant="primary">
          Save changes
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

export default AddCoffee;