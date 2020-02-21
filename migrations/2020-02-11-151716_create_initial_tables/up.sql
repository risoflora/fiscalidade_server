-- Tabela de contribuintes
CREATE TABLE IF NOT EXISTS fiscalidade_taxpayers (
  -- Identificador único
  id BIGSERIAL PRIMARY KEY,
  -- Nome
  name VARCHAR(200) NOT NULL UNIQUE,
  -- Razão Social
  business_name VARCHAR(200) NOT NULL UNIQUE,
  -- CNPJ
  registry VARCHAR(20) NOT NULL UNIQUE,
  -- E-mail
  email VARCHAR(100) NOT NULL UNIQUE,
  -- Certificado PKCS #12
  certificate TEXT NOT NULL,
  -- Senha do certificado
  certificate_password VARCHAR(100) NOT NULL,
  -- Token para login
  token VARCHAR(60) NOT NULL UNIQUE,
  -- Administrador
  manager BOOLEAN NOT NULL DEFAULT FALSE,
  -- Ativo
  active BOOLEAN NOT NULL DEFAULT TRUE,
  -- Data de cadastro
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (name, business_name, registry, email, token)
);

-- Tabela de serviços
CREATE TABLE IF NOT EXISTS fiscalidade_services (
  -- Identificador único
  id BIGSERIAL PRIMARY KEY,
  -- Descrição (NF-e, NFC-e etc.)
  description CHARACTER VARYING(50) NOT NULL UNIQUE,
  -- Serviço (nfe, nfce etc.)
  slug CHARACTER VARYING(50) NOT NULL UNIQUE,
  -- Ativo
  active BOOLEAN NOT NULL DEFAULT TRUE,
  -- Data de cadastro
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (description, slug)
);

-- Tabela para relacionar serviço a contribuinte
CREATE TABLE IF NOT EXISTS fiscalidade_taxpayers_services (
  -- Identificador único
  id BIGSERIAL PRIMARY KEY,
  -- ID do contribuinte
  taxpayer_id BIGINT NOT NULL REFERENCES fiscalidade_taxpayers (id) ON UPDATE CASCADE ON DELETE CASCADE,
  -- ID do serviço
  service_id BIGINT NOT NULL REFERENCES fiscalidade_services (id) ON UPDATE CASCADE ON DELETE CASCADE,
  -- Data de liberação
  allowed_at TIMESTAMP WITHOUT TIME ZONE,
  -- Data de cadastro
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (taxpayer_id, service_id)
);

-- Tabela para fazer caches de conteúdos
CREATE TABLE IF NOT EXISTS fiscalidade_caches (
  -- Identificador único
  id BIGSERIAL PRIMARY KEY,
  -- Localizador de cache
  key VARCHAR(100) NOT NULL UNIQUE,
  -- Conteúdo de cache
  value BYTEA NOT NULL,
  -- Data de cadastro
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insere usuário padrão (admin)
INSERT INTO fiscalidade_taxpayers (
    name,
    business_name,
    registry,
    email,
    certificate,
    certificate_password,
    token,
    manager
  )
VALUES
  (
    'admin',
    'Administrador',
    '',
    '',
    '',
    '',
    'yBtY7BaUiGIHMEXzs1UUdr',
    true
  )
ON CONFLICT (name, business_name, registry, email, token)
DO NOTHING;

-- Insere serviços disponíveis.
INSERT INTO fiscalidade_services (description, slug)
VALUES
  ('NF-e', 'nfe')
ON CONFLICT (description, slug)
DO NOTHING;