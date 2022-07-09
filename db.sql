create table if not exists sys_api_db(
     api_id varchar(32) not null,
     db     varchar(32) not null,
     primary key (api_id, db)
);
create index db_db
    on sys_api_db (db);


create table if not exists sys_dept(
   dept_id    varchar(32) not null,
   parent_id  varchar(32) not null,
   dept_name  varchar(30) not null,
   order_num  int not null,
   leader     varchar(20) null,
   phone      varchar(11) null,
   email      varchar(50) null,
   `status`     varchar(11) not null,
   created_by varchar(32) not null,
   updated_by varchar(32) null,
   created_at datetime not null,
   updated_at datetime  null,
   deleted_at datetime  null
);

create index dept_pid
    on sys_dept (parent_id);

create table if not exists sys_dict_data(
    dict_data_id varchar(32)  not null
        primary key,
    dict_sort    int          not null,
    dict_label   varchar(100) not null,
    dict_value   varchar(100) not null,
    dict_type    varchar(100) not null,
    css_class    varchar(100) null,
    list_class   varchar(100) null,
    is_default   char         not null,
    `status`       char         not null,
    create_by    varchar(32)  not null,
    update_by    varchar(32)  null,
    remark       varchar(500) null,
    created_at   datetime     null,
    updated_at   datetime     null,
    deleted_at   datetime     null
);

create index d_type_tp
    on sys_dict_data (dict_type);


create table if not exists sys_dict_type
(
    dict_type_id varchar(32)  not null
        primary key,
    dict_name    varchar(100) not null,
    dict_type    varchar(100) not null,
    `status`       char         not null,
    create_by    varchar(32)  not null,
    update_by    varchar(32)  null,
    remark       varchar(500) null,
    created_at   datetime     null,
    updated_at   datetime     null,
    deleted_at   datetime     null,
    constraint dict_type
        unique (dict_type)
);

create index d_data_tp
    on sys_dict_type (dict_type);


create table if not exists sys_job
(
    job_id          varchar(32)  not null
        primary key,
    task_id         bigint       not null,
    task_count      bigint       not null,
    run_count       bigint       not null,
    job_name        varchar(64)  not null,
    job_params      varchar(200) null,
    job_group       varchar(64)  not null,
    invoke_target   varchar(500) not null,
    cron_expression varchar(255) not null,
    misfire_policy  char         not null,
    `concurrent`      char         null,
    `status`          char         not null,
    create_by       varchar(32)  not null,
    update_by       varchar(32)  null,
    remark          text         null,
    last_time       datetime     null,
    next_time       datetime     null,
    end_time        datetime     null,
    created_at      datetime     null,
    updated_at      datetime     null,
    deleted_at      datetime     null
);

create index job_tid
    on sys_job (task_id);

create table if not exists sys_job_log
(
    job_log_id     varchar(32)   not null
        primary key,
    job_id         varchar(32)   not null,
    lot_id         bigint        not null,
    lot_order      bigint        not null,
    job_name       varchar(64)   not null,
    job_group      varchar(64)   not null,
    invoke_target  varchar(500)  not null,
    job_params     varchar(500)  null,
    job_message    varchar(500)  null,
    `status`         char          not null,
    exception_info varchar(2000) null,
    is_once        char          null,
    created_at     datetime      not null,
    elapsed_time   bigint        not null
);
create index jobL_jid
    on sys_job_log (job_id);

create table if not exists sys_login_log
(
    info_id        varchar(32)  not null
        primary key,
    login_name     varchar(50)  not null,
    net            varchar(10)  not null,
    ipaddr         varchar(50)  not null,
    login_location varchar(255) not null,
    browser        varchar(50)  not null,
    os             varchar(50)  not null,
    device         varchar(50)  not null,
    `status`         char         not null,
    msg            varchar(255) not null,
    login_time     datetime     not null,
    module         varchar(30)  not null
);

create table if not exists sys_menu
(
    id                varchar(32)  not null
        primary key,
    pid               varchar(32)  not null,
    `path`              varchar(255) not null,
    menu_name         varchar(100) not null,
    icon              varchar(50)  not null,
    menu_type         char         not null,
    `query`             varchar(255) null,
    order_sort        int          not null,
    `status`            char         not null,
    api               varchar(155) not null,
    method            varchar(10)  not null,
    component         varchar(100) not null,
    `visible`           char         not null,
    is_cache          char         not null,
    log_method        char         not null,
    data_cache_method char         not null,
    is_frame          char         not null,
    data_scope        char         not null,
    remark            varchar(255) not null,
    created_at        datetime     null,
    updated_at        datetime     null,
    deleted_at        datetime     null,
    constraint api
        unique (api),
    constraint sm_po
        unique (pid, order_sort)
);

create index sm_method
    on sys_menu (method);

create index sm_mt
    on sys_menu (menu_type);


create table if not exists sys_oper_log
(
    oper_id        varchar(32)   not null
        primary key,
    time_id        bigint        not null,
    title          varchar(50)   not null,
    business_type  char          not null,
    method         varchar(100)  not null,
    request_method varchar(10)   not null,
    operator_type  char          not null,
    oper_name      varchar(50)   not null,
    dept_name      varchar(50)   not null,
    oper_url       varchar(5000) not null,
    oper_ip        varchar(50)   not null,
    oper_location  varchar(255)  not null,
    oper_param     text          not null,
    path_param     text          not null,
    json_result    text          not null,
    `status`         char          not null,
    error_msg      varchar(2000) not null,
    duration       bigint        not null,
    oper_time      datetime      not null
);

create table if not exists sys_post
(
    post_id    varchar(32)  not null
        primary key,
    post_code  varchar(64)  not null,
    post_name  varchar(50)  not null,
    post_sort  int          not null,
    `status`     char         not null,
    remark     varchar(500) null,
    created_by varchar(32)  not null,
    updated_by varchar(32)  null,
    created_at datetime     null,
    updated_at datetime     null,
    deleted_at datetime     null
);

create table if not exists sys_role
(
    role_id    varchar(32)  not null
        primary key,
    role_name  varchar(20)  not null,
    role_key   varchar(100) not null,
    list_order int          not null,
    data_scope char         not null,
    `status`     char         not null,
    remark     varchar(255) null,
    created_at datetime     not null,
    updated_at datetime     null
);

create table if not exists sys_role_api
(
    id         varchar(32)  not null
        primary key,
    role_id    varchar(32)  not null,
    api        varchar(255) not null,
    method     varchar(10)  null,
    created_by varchar(32)  not null,
    created_at datetime     not null
);

create index ra_api
    on sys_role_api (api);

create index ra_api_rid
    on sys_role_api (role_id);


create table if not exists sys_role_dept
(
    role_id    varchar(32) not null,
    dept_id    varchar(32) not null,
    created_at datetime    null,
    primary key (role_id, dept_id)
);

create index rd_dpid
    on sys_role_dept (dept_id);

create index rd_rid
    on sys_role_dept (role_id);


create table if not exists sys_update_log
(
    id              varchar(32)  not null
        primary key,
    app_version     varchar(10)  not null,
    backend_version varchar(10)  not null,
    title           varchar(100) not null,
    content         text         not null,
    created_at      datetime     not null,
    updated_at      datetime     not null,
    deleted_at      datetime     null,
    updated_by      varchar(32)  not null
);

create table if not exists sys_user
(
    id              varchar(32)  not null
        primary key,
    user_name       varchar(60)  not null,
    user_nickname   varchar(50)  not null,
    user_password   varchar(32)  not null,
    user_salt       char(10)     not null,
    user_status     char         not null,
    user_email      varchar(100) null,
    sex             char         not null,
    avatar          varchar(255) not null,
    role_id         varchar(32)  not null,
    dept_id         varchar(32)  not null,
    remark          varchar(255) null,
    is_admin        char         not null,
    phone_num       varchar(20)  null,
    last_login_ip   varchar(15)  null,
    last_login_time datetime     null,
    created_at      datetime     not null,
    updated_at      datetime     null,
    deleted_at      datetime     null,
    constraint user_name
        unique (user_name)
);

create index user_dpid
    on sys_user (dept_id);

create index user_rid
    on sys_user (role_id);

create table if not exists sys_user_dept
(
    id         varchar(32) not null
        primary key,
    user_id    varchar(32) not null,
    dept_id    varchar(32) not null,
    created_by varchar(32) not null,
    created_at datetime    not null
);

create table if not exists sys_user_online
(
    id             varchar(32)  not null
        primary key,
    u_id           varchar(32)  not null,
    token_id       varchar(32)  not null,
    token_exp      bigint       not null,
    login_time     datetime     not null,
    user_name      varchar(255) not null,
    dept_name      varchar(100) not null,
    net            varchar(10)  not null,
    ipaddr         varchar(120) not null,
    login_location varchar(255) not null,
    device         varchar(50)  not null,
    browser        varchar(30)  not null,
    os             varchar(30)  not null
);

create index uo_tid
    on sys_user_online (token_id);

create index uo_uid
    on sys_user_online (u_id);


create table if not exists sys_user_post
(
    user_id    varchar(32) not null,
    post_id    varchar(32) not null,
    created_at datetime    null,
    primary key (user_id, post_id)
);

create table if not exists sys_user_role
(
    id         varchar(32) not null
        primary key,
    user_id    varchar(32) not null,
    role_id    varchar(32) not null,
    created_by varchar(32) not null,
    created_at datetime    not null
);

create index ur_rid
    on sys_user_role (role_id);

create index ur_uid
    on sys_user_role (user_id);