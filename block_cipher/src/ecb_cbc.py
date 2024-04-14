from Crypto.Cipher import AES
from Crypto.Util.Padding import pad, unpad

class ECB:
    def encode(msg,key):
        cipher = AES.new(key, AES.MODE_ECB)
        ciphertext = cipher.encrypt(pad(msg, AES.block_size))
        return ciphertext
    def decode(msg_encoded,key):
        cipher = AES.new(key, AES.MODE_ECB)
        plaintext = unpad(cipher.decrypt(msg_encoded), AES.block_size)
        return plaintext

def cipher_transform(key, start_vector, data, mode):
    result_data = b""
    temp_block = start_vector

    for index in range(0, len(data), AES.block_size):
        current_block = data[index:index + AES.block_size]

        if mode == 'encode':
            current_block = bytes(x ^ y for x, y in zip(current_block, temp_block))
            processed_block = AES.new(key, AES.MODE_ECB).encrypt(current_block)
        else:
            processed_block = AES.new(key, AES.MODE_ECB).decrypt(current_block)
            processed_block = bytes(x ^ y for x, y in zip(processed_block, temp_block))

        result_data += processed_block
        temp_block = processed_block if mode == 'encode' else current_block

    return result_data

def custom_encode(session_key, init_vector, clear_data):
    return cipher_transform(session_key, init_vector, clear_data, 'encode')

def custom_decode(session_key, init_vector, secret_data):
    return cipher_transform(session_key, init_vector, secret_data, 'decode')

plaintext = "hello world"
key = b"123"
iv = b"1234567890123456"
custom_encoded = custom_encode(key, iv, plaintext.encode())
print(custom_encoded)
custom_decoded = custom_decode(key, iv, custom_encoded)
print(custom_decoded.decode())