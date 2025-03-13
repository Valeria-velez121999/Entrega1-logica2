class Nodo {
    int dato;       
    Nodo siguiente; 

    
    public Nodo(int dato) {
        this.dato = dato;
        this.siguiente = null;
    }
}


class ListaSimple {
    private Nodo cabeza; 

    
    public ListaSimple() {
        this.cabeza = null;
    }

    
    public void insertar(int dato) {
        Nodo nuevoNodo = new Nodo(dato);
        if (cabeza == null) {
            cabeza = nuevoNodo; 
        } else {
            Nodo temp = cabeza;
            while (temp.siguiente != null) {
                temp = temp.siguiente; 
            }
            temp.siguiente = nuevoNodo; 
        }
    }

    
    public void mostrar() {
        Nodo temp = cabeza;
        while (temp != null) {
            System.out.print(temp.dato + " -> ");
            temp = temp.siguiente;
        }
        System.out.println("null");
    }
}


class ListaCircular {
    private Nodo cabeza; 

    
    public ListaCircular() {
        this.cabeza = null;
    }

    
    public void insertar(int dato) {
        Nodo nuevoNodo = new Nodo(dato);
        if (cabeza == null) {
            cabeza = nuevoNodo;
            cabeza.siguiente = cabeza; 
        } else {
            Nodo temp = cabeza;
            while (temp.siguiente != cabeza) {
                temp = temp.siguiente; 
            }
            temp.siguiente = nuevoNodo; 
            nuevoNodo.siguiente = cabeza; 
        }
    }

    
    public void mostrar() {
        if (cabeza == null) {
            System.out.println("La lista está vacía");
            return;
        }
        Nodo temp = cabeza;
        do {
            System.out.print(temp.dato + " -> ");
            temp = temp.siguiente;
        } while (temp != cabeza); 
        System.out.println("(vuelve al inicio)");
    }
}

 
public class Main {
    public static void main(String[] args) {
        System.out.println("=== Lista Simple ===");
        ListaSimple listaSimple = new ListaSimple();
        listaSimple.insertar(10);
        listaSimple.insertar(20);
        listaSimple.insertar(30);
        listaSimple.mostrar(); 

        System.out.println("\n=== Lista Circular ===");
        ListaCircular listaCircular = new ListaCircular();
        listaCircular.insertar(100);
        listaCircular.insertar(200);
        listaCircular.insertar(300);
        listaCircular.mostrar(); /
    }
}