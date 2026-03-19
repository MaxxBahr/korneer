import Button from 'react-bootstrap/Button';
import Modal from 'react-bootstrap/Modal';
import AddCoffeeForm from './AddCoffeeForm';

function AddCoffee({ show, handleClose }: {show: boolean, handleClose: ()=> void}) {

  function handleSave(coffeeData: any){
    console.log("Received in parent: ", coffeeData);

    handleClose();
  }

  return (
    <Modal show={show} onHide={handleClose}>
      <Modal.Header closeButton>
        <Modal.Title>Add new coffee</Modal.Title>
      </Modal.Header>

      <Modal.Body>
        <AddCoffeeForm onSave={handleSave}></AddCoffeeForm>
      </Modal.Body>

      <Modal.Footer>
        <Button variant="secondary" onClick={handleClose}>
          Abort
        </Button>

      </Modal.Footer>
    </Modal>
  );
}

export default AddCoffee;