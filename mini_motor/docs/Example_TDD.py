'''
Crear una función llamada “isValidPassword(<String>)” donde van a verificar lo siguiente:

minimo 8 caracters
Minimo una mayúscula
Minimo una minúscula
Mímino un número
No posee espacios

Generar los test.
'''

def isValidPassword(password):
    if not isEnoughLength(password):
        return False
    
    if not isUpperCase(password):
        return False

    if not isLowerCase(password):
        return False
    
    if not hasDigit(password):
        return False
    
    if hasblank(password):
        return False

    return True

def isEnoughLength(password):
    return len(password) >= 8

def isUpperCase(password):
    for l in password:
        if l.isupper():
            return True
    return False

def isLowerCase(password):
    for l in password:
        if ord(l) >= 97 and ord(l) <= 122:
            return True
    return False

def hasDigit(password):
    for l in password:
        if ord(l) >= 48 and ord(l) <= 57:
            return True
    return False

def hasblank(password):
    if " " in password:
        return True
    return False

import unittest

class TestClasificarNumero(unittest.TestCase):
    def test_isValidPassword(self):
        # Los datos 
        casos = [
            ("isShort", False),
            ("islowercase", False),
            ("ISUPPERCASE", False),
            ("Haventdigit", False),
            ("Havent space", False),
            ("Haventdigit8", True),
        ]
        
        for entrada, esperado in casos:
            # Contexto
            with self.subTest(entrada=entrada):
                # Lo que se espera
                self.assertEqual(isValidPassword(entrada), esperado)

    def test_hasblank(self):
        # Los datos 
        casos = [
            ("haventspaceblank", False),
            ("have space blank", True),
        ]
        
        for entrada, esperado in casos:
            # Contexto
            with self.subTest(entrada=entrada):
                # Lo que se espera
                self.assertEqual(hasblank(entrada), esperado)

    def test_hasDigit(self):
        # Los datos 
        casos = [
            ("haventdigit", False),
            ("havedigit8", True),
        ]
        
        for entrada, esperado in casos:
            # Contexto
            with self.subTest(entrada=entrada):
                # Lo que se espera
                self.assertEqual(hasDigit(entrada), esperado)

    def test_isLower(self):
        # Los datos 
        casos = [
            ("ISUPPERCASE", False),
            ("islowercase", True),
        ]
        
        for entrada, esperado in casos:
            # Contexto
            with self.subTest(entrada=entrada):
                # Lo que se espera
                self.assertEqual(isLowerCase(entrada), esperado)

    def test_isUpper(self):
        # Los datos 
        casos = [
            ("isUppercase", True),
            ("islowercase", False),
        ]
        
        for entrada, esperado in casos:
            # Contexto
            with self.subTest(entrada=entrada):
                # Lo que se espera
                self.assertEqual(isUpperCase(entrada), esperado)

    def test_isEnoughLength(self):
        # Los datos 
        casos = [
            ("", False ),
            ("isShort", False),
            ("isEnough", True),
            ("isMaxEight", True)
        ]
        
        for entrada, esperado in casos:
            # Contexto
            with self.subTest(entrada=entrada):
                # Lo que se espera
                self.assertEqual(isEnoughLength(entrada), esperado)

if __name__ == '__main__':
    unittest.main()

