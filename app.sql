CREATE TABLE categoies ( -- 分类
	id SERIAL PRIMARY KEY, -- 自增主键
	name VARCHAR(20) NOT NULL UNIQUE, -- 分类名称
	is_del BOOLEAN NOT NULL DEFAULT FALSE -- 是否删除
);

CREATE TABLE articles ( -- 文章
	id SERIAL PRIMARY KEY, -- 自增主键
	category_id INT NOT NULL REFERENCES categoies(id), -- 文章所属分类的ID，外键
	title VARCHAR(255) NOT NULL, -- 文章标题
	content TEXT NOT NULL, -- 文章内容
	dateline TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 添加时间
	is_del BOOLEAN NOT NULL DEFAULT FALSE -- 是否删除
);

