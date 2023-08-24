/* Lei de Ohm e seu principio
 *
 * V = Voltagem(tensão) | I = Corrente(Ampere) | R = Resistência(Ω)
 *
 * Voltagem(tensão) é igual à corrente(Ampere) 
 * multiplicado pela resistência(Ohm Ω)
 *
 * Resistência é igual a Voltagem(tensão)
 * dividido pela corrente
 *
 * Corrente é igual a Resistência 
 * dividido pela Voltagem(tensão)
 *
 * Fórmula: V = I * R
 * Fórmula: R = V / I
 * Fórmula: I = V / R
 */

fn ler_tensao() -> f64 {

    const MAX_TENTATIVAS: usize = 3;
    let mut tentativas = 0;
    loop {
        if tentativas >= MAX_TENTATIVAS {
            println!("Número Máximo de Tentativas Excedida! Usando valor padrão de 0.0!");
            return 0.0;
        }
    

        println!("Informe a Voltagem: "); 
        let mut voltagem = String::new();
        std::io::stdin()
        .read_line(&mut voltagem)
        .expect("Valor Inváldo. Gentileza utilizar apenas Números");

        let n_voltagem: f64 = match voltagem.trim().parse(){
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Erro ao converter a voltagem, caso seja um número inteiro
                colocar .0 no final.. Exemplo: 3.0");
                tentativas += 1;
                continue; //isso retorna para o inicio da função, caso aja um erro na entrada de dados           
            }            
        };
    return n_voltagem;
    }
    
}

fn ler_corrente() -> f64 {   

    const MAX_TENTATIVAS: usize = 3;
    let mut tentativas = 0;
    loop {
            if tentativas >= MAX_TENTATIVAS {
                println!("Número Máximo de Tentativas Excedida! Usando valor padrão de 0.0!");
                return 0.0;
            }
        

        println!("informe o valor da corrente: ");
        let mut corrente = String::new();
        std::io::stdin()
        .read_line(&mut corrente)
        .expect("Falha ao ler entrada");

        let mut n_corrente: f64 = match corrente.trim().parse(){
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Erro ao converter a corrente, caso seja um número inteiro
                colocar .0 no final... Exemplo: 3.0");
                tentativas += 1;
                continue;//isso retorna para o inicio da função, caso aja um erro na entrada de dados           
            }            
        };
        
        //Conversão de mA para A
        println!("O valor informado está em MiliAmpere(mA)?
        Digite 'S' para Sim, ou digite qualquer tecla para Não.");
        let mut resposta = String::new();
        std::io::stdin()
        .read_line(& mut resposta)
        .expect("Falha ao ler Entrada");

        let n_resposta = resposta.trim().to_uppercase();
        if n_resposta == "S" {
            n_corrente /= 1000.0;
        }  
    return n_corrente;  
    }  
         
}

fn ler_resistencia() -> f64 {
    const MAX_TENTATIVAS: usize = 3;
    let mut tentativas = 0;

    loop{        
        if tentativas >= MAX_TENTATIVAS {
            println!("Número Máximo de Tentativas Excedida! Usando valor padrão de 0.0!");
            return 0.0;
        }
    
        print!("Informe o valor da Resistência: \n");
        let mut resistencia = String::new();
        std::io::stdin().read_line(&mut resistencia).expect("Falha ao ler Entrada");

        let n_resistencia = match resistencia.trim().parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                println!("Erro ao converter a corrente, caso seja um número inteiro
                colocar .0 no final... Exemplo: 3.0");
                tentativas += 1;
                continue;
            }
        };
    return n_resistencia;
    }
        

}


fn main() {
    println!("Informe o valor:\n1 - Resistência(Ohm's)\t2 - Tensão(Volts)\t3 - Corrente");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Falha ao ler Entrada");

    let n_input: u32 = input.trim().parse()
    .expect("Falha ao ler Entrada");

    match n_input {
        1 => {
            let v = &ler_tensao;
            let i = &ler_corrente;
            let resultado = v() / i();
            println!("O Valor da Resistência é de {}Ohm's", resultado);
        }
        2 => {
            let i = &ler_corrente;
            let r = &ler_resistencia;
            let resultado = i() * r();
            print!("O Valor da Tensão é de {} Volts", resultado);
        }
        3=> {
            let v = &ler_tensao;
            let r = &ler_resistencia;
            let resultado = v() / r();
            let m_a_resultado = resultado * 1000.0; //MiliAmpere
            print!("O Valor da Corrente é de {}A ou {}mA", resultado, m_a_resultado);
        }
        _ => println!("Falha ao ler Entrada"),
    }
}
