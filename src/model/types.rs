use rustc_hash::FxHashSet;
use std::hash::{Hash, Hasher};
use std::{fmt, ops::Deref};

use super::SyscallRef;

/// Unique id of each type.
pub type TypeId = usize;

/// Type information.
#[derive(Debug)]
pub struct Type {
    pub id: TypeId,
    pub name: String,
    pub sz: u64,
    pub align: u64,
    pub optional: bool,
    pub varlen: bool,
    pub kind: TypeKind,
}

impl Type {
    pub fn template_name(&self) -> &str {
        let name = &*self.name;
        if let Some(idx) = name.find('[') {
            &name[0..idx]
        } else {
            name
        }
    }

    pub fn is_optional(&self) -> bool {
        self.optional
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_varlen(&self) -> bool {
        self.varlen
    }

    pub fn alignment(&self) -> u64 {
        self.align
    }

    pub fn size(&self) -> u64 {
        self.sz
    }

    pub fn is_res_kind(&self) -> bool {
        matches!(&self.kind, TypeKind::Res { .. })
    }

    pub fn res_desc(&self) -> Option<&ResDesc> {
        match &self.kind {
            TypeKind::Res { desc, .. } => Some(desc),
            _ => None,
        }
    }

    pub fn res_desc_mut(&mut self) -> Option<&mut ResDesc> {
        match &mut self.kind {
            TypeKind::Res { desc, .. } => Some(desc),
            _ => None,
        }
    }

    pub fn is_buffer_kind(&self) -> bool {
        matches!(&self.kind, TypeKind::Buffer { .. })
    }

    pub fn buffer_kind(&self) -> Option<&BufferKind> {
        match &self.kind {
            TypeKind::Buffer { kind, .. } => Some(kind),
            _ => None,
        }
    }

    pub fn buffer_kind_mut(&mut self) -> Option<&mut BufferKind> {
        match &mut self.kind {
            TypeKind::Buffer { kind, .. } => Some(kind),
            _ => None,
        }
    }

    pub fn ptr_info(&self) -> Option<(TypeRef, Dir)> {
        match &self.kind {
            TypeKind::Ptr { elem, dir } => Some((*elem, *dir)),
            _ => None,
        }
    }

    pub fn array_info(&self) -> Option<(TypeRef, Option<(u64, u64)>)> {
        match &self.kind {
            TypeKind::Array { elem, range } => Some((*elem, *range)),
            _ => None,
        }
    }

    pub fn fields(&self) -> Option<&[Field]> {
        match &self.kind {
            TypeKind::Struct { fields, .. } | TypeKind::Union { fields } => Some(fields),
            _ => None,
        }
    }

    pub fn len_info(&self) -> Option<&LenInfo> {
        match &self.kind {
            TypeKind::Len { len_info, .. } => Some(len_info),
            _ => None,
        }
    }

    pub fn int_fmt(&self) -> Option<&IntFmt> {
        match &self.kind {
            TypeKind::Int { int_fmt, .. }
            | TypeKind::Flags { int_fmt, .. }
            | TypeKind::Csum { int_fmt, .. }
            | TypeKind::Proc { int_fmt, .. }
            | TypeKind::Const { int_fmt, .. }
            | TypeKind::Len { int_fmt, .. } => Some(int_fmt),
            _ => None,
        }
    }

    pub fn bf_offset(&self) -> u64 {
        if let Some(int_fmt) = self.int_fmt() {
            int_fmt.bitfield_off
        } else {
            0
        }
    }

    pub fn bf_len(&self) -> u64 {
        if let Some(int_fmt) = self.int_fmt() {
            int_fmt.bitfield_len
        } else {
            0
        }
    }

    pub fn bin_fmt(&self) -> BinFmt {
        match &self.kind {
            TypeKind::Int { int_fmt, .. }
            | TypeKind::Flags { int_fmt, .. }
            | TypeKind::Csum { int_fmt, .. }
            | TypeKind::Proc { int_fmt, .. }
            | TypeKind::Const { int_fmt, .. }
            | TypeKind::Len { int_fmt, .. } => int_fmt.fmt,
            TypeKind::Res { fmt, .. } => *fmt,
            _ => BinFmt::Native,
        }
    }

    pub fn unit_offset(&self) -> u64 {
        if let Some(int_fmt) = self.int_fmt() {
            int_fmt.bitfield_unit_off
        } else {
            0
        }
    }

    pub fn is_bitfield(&self) -> bool {
        if let Some(int_fmt) = self.int_fmt() {
            int_fmt.bitfield_len != 0
        } else {
            false
        }
    }

    pub fn bit_len(&self) -> Option<u64> {
        if let Some(int_fmt) = self.int_fmt() {
            if int_fmt.fmt != BinFmt::Native && int_fmt.fmt != BinFmt::BigEndian {
                Some(64)
            } else if int_fmt.bitfield_len != 0 {
                Some(int_fmt.bitfield_len)
            } else {
                Some(self.size())
            }
        } else {
            None
        }
    }

    pub fn is_readable_date_type(&self) -> bool {
        match &self.kind {
            TypeKind::Buffer { kind, .. } => {
                matches!(
                    kind,
                    BufferKind::String { .. } | BufferKind::Filename { .. }
                )
            }
            _ => false,
        }
    }

    pub fn is_pad(&self) -> bool {
        if let TypeKind::Const { pad, .. } = &self.kind {
            *pad
        } else {
            false
        }
    }

    pub fn is_str_like(&self) -> bool {
        if let TypeKind::Buffer { kind, .. } = &self.kind {
            matches!(
                kind,
                BufferKind::Filename { .. } | BufferKind::String { .. }
            )
        } else {
            false
        }
    }
}

/// Order by TypeId
impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Type) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Type) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/// Compare by TypeId
impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        self.id == other.id
    }
}

impl Eq for Type {}

/// Hash by TypeId
impl Hash for Type {
    fn hash<T: Hasher>(&self, h: &mut T) {
        h.write_usize(self.id);
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Ref of another type.
/// Id ref is only used during constructing target.
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub enum TypeRef {
    /// Temporary ref representation for convenience, used only during parsing json representation.
    Id(TypeId),
    /// Share ref.
    Ref(&'static Type),
}

impl TypeRef {
    pub fn as_id(&self) -> Option<TypeId> {
        if let TypeRef::Id(id) = self {
            Some(*id)
        } else {
            None
        }
    }
}

impl Deref for TypeRef {
    type Target = Type;
    fn deref(&self) -> &Type {
        match self {
            TypeRef::Ref(ty) => ty,
            _ => panic!("typeref was derefed, while owning typeid"),
        }
    }
}

impl fmt::Display for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeRef::Ref(ty) => write!(f, "{}", ty),
            TypeRef::Id(id) => write!(f, "id({})", id),
        }
    }
}

/// Type kind.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TypeKind {
    Res {
        fmt: BinFmt,
        desc: ResDesc,
    },
    Const {
        int_fmt: IntFmt,
        val: u64,
        pad: bool,
    },
    Int {
        int_fmt: IntFmt,
        range: Option<(u64, u64)>,
        align: u64,
    },
    Flags {
        int_fmt: IntFmt,
        vals: Vec<u64>,
        bitmask: bool,
    },
    Len {
        int_fmt: IntFmt,
        len_info: LenInfo,
    },
    Proc {
        int_fmt: IntFmt,
        start: u64,
        per_proc: u64,
    },
    Csum {
        int_fmt: IntFmt,
        kind: CsumKind,
        buf: String,
        protocol: u64,
    },
    Vma {
        begin: u64,
        end: u64,
    },
    Buffer {
        kind: BufferKind,
        subkind: Option<String>,
    },
    Array {
        range: Option<(u64, u64)>,
        elem: TypeRef,
    },
    Ptr {
        elem: TypeRef,
        dir: Dir,
    },
    Struct {
        fields: Vec<Field>,
        align_attr: u64,
    },
    Union {
        fields: Vec<Field>,
    },
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Field {
    /// Name of Field.
    pub name: String,
    /// TypeRef of Field.
    pub ty: TypeRef,
    /// Direction of field.
    pub dir: Option<Dir>,
}

impl TypeKind {
    pub fn new_res(fmt: BinFmt, desc: ResDesc) -> Self {
        TypeKind::Res { fmt, desc }
    }

    pub fn new_const(int_fmt: IntFmt, val: u64, pad: bool) -> Self {
        TypeKind::Const { int_fmt, val, pad }
    }

    pub fn new_int(int_fmt: IntFmt, range: Option<(u64, u64)>, align: u64) -> Self {
        TypeKind::Int {
            int_fmt,
            range,
            align,
        }
    }

    pub fn new_len(int_fmt: IntFmt, len_info: LenInfo) -> Self {
        TypeKind::Len { int_fmt, len_info }
    }

    pub fn new_flags(int_fmt: IntFmt, vals: Vec<u64>, bitmask: bool) -> Self {
        TypeKind::Flags {
            int_fmt,
            vals,
            bitmask,
        }
    }

    pub fn new_csum(int_fmt: IntFmt, kind: CsumKind, buf: String, protocol: u64) -> Self {
        TypeKind::Csum {
            int_fmt,
            kind,
            buf,
            protocol,
        }
    }

    pub fn new_proc(int_fmt: IntFmt, start: u64, per_proc: u64) -> Self {
        TypeKind::Proc {
            int_fmt,
            start,
            per_proc,
        }
    }

    pub fn new_buffer(kind: BufferKind, subkind: String) -> Self {
        TypeKind::Buffer {
            kind,
            subkind: if subkind.is_empty() {
                None
            } else {
                Some(subkind)
            },
        }
    }

    pub fn new_vma(begin: u64, end: u64) -> Self {
        TypeKind::Vma { begin, end }
    }

    pub fn new_ptr(elem: TypeId, dir: Dir) -> Self {
        TypeKind::Ptr {
            elem: TypeRef::Id(elem),
            dir,
        }
    }

    pub fn new_array(elem: TypeId, range: Option<(u64, u64)>) -> Self {
        TypeKind::Array {
            range,
            elem: TypeRef::Id(elem),
        }
    }

    pub fn new_struct(align: u64, fields: Vec<Field>) -> Self {
        TypeKind::Struct {
            fields,
            align_attr: align,
        }
    }

    pub fn new_union(fields: Vec<Field>) -> Self {
        TypeKind::Union { fields }
    }

    pub fn new_void() -> Self {
        Self::Buffer {
            kind: BufferKind::BlobRange(0, 0),
            subkind: None,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct LenInfo {
    pub bit_sz: u64,
    pub offset: bool,
    pub path: Vec<String>,
}

impl LenInfo {
    pub fn new(bit_sz: u64, offset: bool, path: Vec<String>) -> Self {
        Self {
            bit_sz,
            offset,
            path,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ResDesc {
    /// Name of resource.
    pub name: String,
    /// Subkind of these kind resource.
    pub kinds: Vec<String>,
    /// Special value for current resource.
    pub vals: Vec<u64>,
    /// Underlying type, None value for default u64.
    pub ty: Option<TypeRef>,
    /// Possible constructors.
    pub ctors: FxHashSet<SyscallRef>,
    /// Possible consumers.
    pub consumers: FxHashSet<SyscallRef>,
}

impl ResDesc {
    pub fn new(name: String, kinds: Vec<String>, vals: Vec<u64>) -> Self {
        ResDesc {
            name,
            kinds,
            vals,
            ty: None,
            ctors: FxHashSet::default(),
            consumers: FxHashSet::default(),
        }
    }
}

/// Binary format.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy)]
pub enum BinFmt {
    Native = 0,
    BigEndian,
    StrDec,
    StrHex,
    StrOct,
}

impl From<u64> for BinFmt {
    fn from(val: u64) -> Self {
        match val {
            0 => BinFmt::Native,
            1 => BinFmt::BigEndian,
            2 => BinFmt::StrDec,
            3 => BinFmt::StrHex,
            4 => BinFmt::StrOct,
            _ => panic!("bad bin fmt value: {}", val),
        }
    }
}

impl fmt::Display for BinFmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = match self {
            Self::Native => "Native",
            Self::BigEndian => "BigEndian",
            Self::StrDec => "StrDec",
            Self::StrHex => "StrHex",
            Self::StrOct => "StrOct",
        };
        write!(f, "{}", b)
    }
}

/// Integer format.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct IntFmt {
    pub fmt: BinFmt,
    pub bitfield_off: u64,
    pub bitfield_len: u64,
    pub bitfield_unit: u64,
    pub bitfield_unit_off: u64,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum CsumKind {
    Inet = 0,
    Pseudo,
}

impl From<u64> for CsumKind {
    fn from(val: u64) -> Self {
        match val {
            0 => CsumKind::Inet,
            1 => CsumKind::Pseudo,
            _ => panic!("bad csumkind: {}", val),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum BufferKind {
    BlobRand,
    BlobRange(u64, u64),
    String { vals: Vec<Vec<u8>>, noz: bool },
    Filename { vals: Vec<Vec<u8>>, noz: bool },
    Text(TextKind),
    BufferGlob,
}

impl BufferKind {
    pub fn new_blob(range: Option<(u64, u64)>) -> Self {
        if let Some(range) = range {
            BufferKind::BlobRange(range.0, range.1)
        } else {
            BufferKind::BlobRand
        }
    }

    pub fn new_str(vals: Vec<Vec<u8>>, noz: bool) -> Self {
        BufferKind::String { vals, noz }
    }

    pub fn new_fname(vals: Vec<Vec<u8>>, noz: bool) -> Self {
        BufferKind::Filename { vals, noz }
    }

    pub fn new_text(kind: TextKind) -> Self {
        BufferKind::Text(kind)
    }

    pub fn new_glob() -> Self {
        BufferKind::BufferGlob
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum TextKind {
    Target = 0,
    X86Real,
    X86bit16,
    X86bit32,
    X86bit64,
    Arm64,
    Ppc64,
}

impl From<u64> for TextKind {
    fn from(val: u64) -> Self {
        match val {
            0 => TextKind::Target,
            1 => TextKind::X86Real,
            2 => TextKind::X86bit16,
            3 => TextKind::X86bit32,
            4 => TextKind::X86bit64,
            5 => TextKind::Arm64,
            6 => TextKind::Ppc64,
            _ => panic!("bad text kind: {}", val),
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone, Copy, Hash)]
pub enum Dir {
    In = 0,
    Out,
    InOut,
}

impl From<u64> for Dir {
    fn from(val: u64) -> Self {
        match val {
            0 => Dir::In,
            1 => Dir::Out,
            2 => Dir::InOut,
            _ => panic!("bad dir value: {}", val),
        }
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p = match self {
            Self::In => "In",
            Self::Out => "Out",
            Self::InOut => "InOut",
        };
        write!(f, "{}", p)
    }
}
